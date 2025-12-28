macro_rules! deps {
    () => {
        LineProgramHeader!();
        ReaderOffset!();
        FileEntry!();
        AttributeValue!();
        Reader!();
        Result!();
    };
}

macro_rules! impl_428 {
    () => {
        deps!();
        impl < R , Offset > FileEntry < R , Offset > where R : Reader < Offset = Offset > , Offset : ReaderOffset , { fn parse (input : & mut R , path_name : R) -> Result < FileEntry < R , Offset > > { let directory_index = input . read_uleb128 () ? ; let timestamp = input . read_uleb128 () ? ; let size = input . read_uleb128 () ? ; let entry = FileEntry { path_name : AttributeValue :: String (path_name) , directory_index , timestamp , size , md5 : [0 ; 16] , source : None , } ; Ok (entry) } # [doc = " > A slice containing the full or relative path name of"] # [doc = " > a source file. If the entry contains a file name or a relative path"] # [doc = " > name, the file is located relative to either the compilation directory"] # [doc = " > (as specified by the DW_AT_comp_dir attribute given in the compilation"] # [doc = " > unit) or one of the directories in the include_directories section."] pub fn path_name (& self) -> AttributeValue < R , Offset > { self . path_name . clone () } # [doc = " > An unsigned LEB128 number representing the directory index of the"] # [doc = " > directory in which the file was found."] # [doc = " >"] # [doc = " > ..."] # [doc = " >"] # [doc = " > The directory index represents an entry in the include_directories"] # [doc = " > section of the line number program header. The index is 0 if the file"] # [doc = " > was found in the current directory of the compilation, 1 if it was found"] # [doc = " > in the first directory in the include_directories section, and so"] # [doc = " > on. The directory index is ignored for file names that represent full"] # [doc = " > path names."] pub fn directory_index (& self) -> u64 { self . directory_index } # [doc = " Get this file's directory."] # [doc = ""] # [doc = " A directory index of 0 corresponds to the compilation unit directory."] pub fn directory (& self , header : & LineProgramHeader < R >) -> Option < AttributeValue < R , Offset > > { header . directory (self . directory_index) } # [doc = " The implementation-defined time of last modification of the file,"] # [doc = " or 0 if not available."] pub fn timestamp (& self) -> u64 { self . timestamp } # [doc = " \"An unsigned LEB128 number representing the time of last modification of"] # [doc = " the file, or 0 if not available.\""] # [doc (hidden)] pub fn last_modification (& self) -> u64 { self . timestamp } # [doc = " The size of the file in bytes, or 0 if not available."] pub fn size (& self) -> u64 { self . size } # [doc = " \"An unsigned LEB128 number representing the length in bytes of the file,"] # [doc = " or 0 if not available.\""] # [doc (hidden)] pub fn length (& self) -> u64 { self . size } # [doc = " A 16-byte MD5 digest of the file contents."] # [doc = ""] # [doc = " Only valid if `LineProgramHeader::file_has_md5` returns `true`."] pub fn md5 (& self) -> & [u8 ; 16] { & self . md5 } # [doc = " The source code of this file. (UTF-8 source text string with \"\\n\" line"] # [doc = " endings)."] # [doc = ""] # [doc = " Note: For DWARF v5 files this may return an empty attribute that"] # [doc = " indicates that no source code is available, which this function"] # [doc = " represents as `Some(<zero-length attr>)`."] pub fn source (& self) -> Option < AttributeValue < R , Offset > > { self . source . clone () } }
    };
}

impl_428!()