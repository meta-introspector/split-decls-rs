macro_rules! deps {
    () => {
        U32!();
        U16!();
    };
}

macro_rules! FileHeader32 {
    () => {
        deps!();
        # [doc = " The header at the start of every 32-bit XCOFF file."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct FileHeader32 { # [doc = " Magic number. Must be 0x01DF."] pub f_magic : U16 < BE > , # [doc = " Number of sections."] pub f_nscns : U16 < BE > , # [doc = " Time and date of file creation."] pub f_timdat : U32 < BE > , # [doc = " Byte offset to symbol table start."] pub f_symptr : U32 < BE > , # [doc = " Number of entries in symbol table."] pub f_nsyms : U32 < BE > , # [doc = " Number of bytes in optional header"] pub f_opthdr : U16 < BE > , # [doc = " Extra flags."] pub f_flags : U16 < BE > , }
    };
}

FileHeader32!()