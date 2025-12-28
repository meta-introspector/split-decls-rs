macro_rules! deps {
    () => {
        LineProgram!();
        LineString!();
    };
}

macro_rules! FileInfo {
    () => {
        deps!();
        # [doc = " Extra information for file in a `LineProgram`."] # [derive (Debug , Default , Clone , PartialEq , Eq)] pub struct FileInfo { # [doc = " The implementation defined timestamp of the last modification of the file,"] # [doc = " or 0 if not available."] pub timestamp : u64 , # [doc = " The size of the file in bytes, or 0 if not available."] pub size : u64 , # [doc = " A 16-byte MD5 digest of the file contents."] # [doc = ""] # [doc = " Only used if version >= 5 and `LineProgram::file_has_md5` is `true`."] pub md5 : [u8 ; 16] , # [doc = " Optionally some embedded sourcecode."] # [doc = ""] # [doc = " Only used if version >= 5 and `LineProgram::file_has_source` is `true`."] pub source : Option < LineString > , }
    };
}

FileInfo!();