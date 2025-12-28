macro_rules! deps {
    () => {
        FileKind!();
        ReadRef!();
        File!();
        Result!();
        MachO32!();
        Error!();
        DyldCache!();
        MachO64!();
    };
}

macro_rules! impl_898 {
    () => {
        deps!();
        impl FileKind { # [doc = " Determine a file kind by parsing the start of the file."] pub fn parse < 'data , R : ReadRef < 'data > > (data : R) -> Result < FileKind > { Self :: parse_at (data , 0) } # [doc = " Determine a file kind by parsing at the given offset."] pub fn parse_at < 'data , R : ReadRef < 'data > > (data : R , offset : u64) -> Result < FileKind > { let magic = data . read_bytes_at (offset , 16) . read_error ("Could not read file magic") ? ; if magic . len () < 16 { return Err (Error ("File too short")) ; } let kind = match [magic [0] , magic [1] , magic [2] , magic [3] , magic [4] , magic [5] , magic [6] , magic [7]] { # [cfg (feature = "archive")] [b'!' , b'<' , b'a' , b'r' , b'c' , b'h' , b'>' , b'\n'] | [b'!' , b'<' , b't' , b'h' , b'i' , b'n' , b'>' , b'\n'] => FileKind :: Archive , # [cfg (feature = "macho")] [b'd' , b'y' , b'l' , b'd' , b'_' , b'v' , b'1' , b' '] => FileKind :: DyldCache , # [cfg (feature = "elf")] [0x7f , b'E' , b'L' , b'F' , 1 , ..] => FileKind :: Elf32 , # [cfg (feature = "elf")] [0x7f , b'E' , b'L' , b'F' , 2 , ..] => FileKind :: Elf64 , # [cfg (feature = "macho")] [0xfe , 0xed , 0xfa , 0xce , ..] | [0xce , 0xfa , 0xed , 0xfe , ..] => FileKind :: MachO32 , # [cfg (feature = "macho")] | [0xfe , 0xed , 0xfa , 0xcf , ..] | [0xcf , 0xfa , 0xed , 0xfe , ..] => FileKind :: MachO64 , # [cfg (feature = "macho")] [0xca , 0xfe , 0xba , 0xbe , ..] => FileKind :: MachOFat32 , # [cfg (feature = "macho")] [0xca , 0xfe , 0xba , 0xbf , ..] => FileKind :: MachOFat64 , # [cfg (feature = "wasm")] [0x00 , b'a' , b's' , b'm' , _ , _ , 0x00 , 0x00] => FileKind :: Wasm , # [cfg (feature = "pe")] [b'M' , b'Z' , ..] if offset == 0 => { match pe :: optional_header_magic (data) { Ok (crate :: pe :: IMAGE_NT_OPTIONAL_HDR32_MAGIC) => { FileKind :: Pe32 } Ok (crate :: pe :: IMAGE_NT_OPTIONAL_HDR64_MAGIC) => { FileKind :: Pe64 } _ => return Err (Error ("Unknown MS-DOS file")) , } } # [cfg (feature = "coff")] [0xc4 , 0x01 , ..] | [0x64 , 0xaa , ..] | [0x41 , 0xa6 , ..] | [0xf0 , 0x01 , ..] | [0xf1 , 0x01 , ..] | [0xf2 , 0x01 , ..] | [0x4c , 0x01 , ..] | [0x64 , 0x86 , ..] => FileKind :: Coff , # [cfg (feature = "coff")] [0x00 , 0x00 , 0xff , 0xff , 0x00 , 0x00 , ..] => FileKind :: CoffImport , # [cfg (feature = "coff")] [0x00 , 0x00 , 0xff , 0xff , 0x02 , 0x00 , ..] if offset == 0 => { match coff :: anon_object_class_id (data) { Ok (crate :: pe :: ANON_OBJECT_HEADER_BIGOBJ_CLASS_ID) => FileKind :: CoffBig , _ => return Err (Error ("Unknown anon object file")) , } } # [cfg (feature = "xcoff")] [0x01 , 0xdf , ..] => FileKind :: Xcoff32 , # [cfg (feature = "xcoff")] [0x01 , 0xf7 , ..] => FileKind :: Xcoff64 , _ => return Err (Error ("Unknown file magic")) , } ; Ok (kind) } }
    };
}

impl_898!()