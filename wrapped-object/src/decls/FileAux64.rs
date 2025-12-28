macro_rules! deps {
    () => {
        Symbols!();
        File!();
    };
}

macro_rules! FileAux64 {
    () => {
        deps!();
        # [doc = " File Auxiliary Entry for C_FILE Symbols."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct FileAux64 { # [doc = " The source file name or compiler-related string."] # [doc = ""] # [doc = " If first 4 bytes are 0, then second 4 bytes are offset into string table."] pub x_fname : [u8 ; 8] , # [doc = " Pad size for file name."] pub x_fpad : [u8 ; 6] , # [doc = " The source-file string type."] pub x_ftype : u8 , # [doc = " Reserved."] pub x_freserve : [u8 ; 2] , # [doc = " Specifies the type of auxiliary entry. Contains _AUX_FILE for this auxiliary entry."] pub x_auxtype : u8 , }
    };
}

FileAux64!()