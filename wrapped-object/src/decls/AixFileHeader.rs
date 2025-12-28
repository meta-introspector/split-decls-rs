macro_rules! AixFileHeader {
    () => {
        # [doc = " The AIX big archive's fixed length header at file beginning."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct AixFileHeader { # [doc = " Archive magic string."] pub magic : [u8 ; 8] , # [doc = " Offset of member table."] pub memoff : [u8 ; 20] , # [doc = " Offset of global symbol table."] pub gstoff : [u8 ; 20] , # [doc = " Offset of global symbol table for 64-bit objects."] pub gst64off : [u8 ; 20] , # [doc = " Offset of first member."] pub fstmoff : [u8 ; 20] , # [doc = " Offset of last member."] pub lstmoff : [u8 ; 20] , # [doc = " Offset of first member on free list."] pub freeoff : [u8 ; 20] , }
    };
}

AixFileHeader!()