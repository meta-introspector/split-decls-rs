macro_rules! deps {
    () => {
        SubArchitecture!();
        ReadRef!();
        ImportName!();
        Error!();
        Name!();
        ImportObjectHeader!();
        ByteString!();
        Result!();
        ImportType!();
        Architecture!();
        ImportFile!();
    };
}

macro_rules! impl_271 {
    () => {
        deps!();
        impl < 'data > ImportFile < 'data > { # [doc = " Parse it."] pub fn parse < R : ReadRef < 'data > > (data : R) -> Result < Self > { let mut offset = 0 ; let header = pe :: ImportObjectHeader :: parse (data , & mut offset) ? ; let data = header . parse_data (data , & mut offset) ? ; fn strip_prefix (s : & [u8]) -> & [u8] { match s . split_first () { Some ((b , rest)) if [b'?' , b'@' , b'_'] . contains (b) => rest , _ => s , } } Ok (Self { header , dll : data . dll , symbol : data . symbol , kind : match header . import_type () { pe :: IMPORT_OBJECT_CODE => ImportType :: Code , pe :: IMPORT_OBJECT_DATA => ImportType :: Data , pe :: IMPORT_OBJECT_CONST => ImportType :: Const , _ => return Err (Error ("Invalid COFF import library import type")) , } , import : match header . name_type () { pe :: IMPORT_OBJECT_ORDINAL => None , pe :: IMPORT_OBJECT_NAME => Some (data . symbol ()) , pe :: IMPORT_OBJECT_NAME_NO_PREFIX => Some (strip_prefix (data . symbol ())) , pe :: IMPORT_OBJECT_NAME_UNDECORATE => Some (strip_prefix (data . symbol ()) . split (| & b | b == b'@') . next () . unwrap () ,) , pe :: IMPORT_OBJECT_NAME_EXPORTAS => data . export () , _ => return Err (Error ("Unknown COFF import library name type")) , } . map (ByteString) , }) } # [doc = " Get the machine type."] pub fn architecture (& self) -> Architecture { match self . header . machine . get (LE) { pe :: IMAGE_FILE_MACHINE_ARMNT => Architecture :: Arm , pe :: IMAGE_FILE_MACHINE_ARM64 | pe :: IMAGE_FILE_MACHINE_ARM64EC => Architecture :: Aarch64 , pe :: IMAGE_FILE_MACHINE_I386 => Architecture :: I386 , pe :: IMAGE_FILE_MACHINE_AMD64 => Architecture :: X86_64 , _ => Architecture :: Unknown , } } # [doc = " Get the sub machine type, if available."] pub fn sub_architecture (& self) -> Option < SubArchitecture > { match self . header . machine . get (LE) { pe :: IMAGE_FILE_MACHINE_ARM64EC => Some (SubArchitecture :: Arm64EC) , _ => None , } } # [doc = " The public symbol name."] pub fn symbol (& self) -> & 'data [u8] { self . symbol . 0 } # [doc = " The name of the DLL to import the symbol from."] pub fn dll (& self) -> & 'data [u8] { self . dll . 0 } # [doc = " The name exported from the DLL."] pub fn import (& self) -> ImportName < 'data > { match self . import { Some (name) => ImportName :: Name (name . 0) , None => ImportName :: Ordinal (self . header . ordinal_or_hint . get (LE)) , } } # [doc = " The type of import. Usually either a function or data."] pub fn import_type (& self) -> ImportType { self . kind } }
    };
}

impl_271!()