// Generated macro for impl_51 (impl)
macro_rules! Depcrate_bufreadimpl_51 {
() => {
// Module: crate::bufread
// Provides: {"impl_51"}
// Dependencies: {}
impl < R : BufRead > Read for XzDecoder < R > { # [inline] fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { if buf . is_empty () { return Ok (0) ; } loop { let (read , consumed , eof , ret) ; { let input = self . obj . fill_buf () ? ; eof = input . is_empty () ; let before_out = self . data . total_out () ; let before_in = self . data . total_in () ; ret = self . data . process (input , buf , if eof { Action :: Finish } else { Action :: Run }) ; read = (self . data . total_out () - before_out) as usize ; consumed = (self . data . total_in () - before_in) as usize ; } self . obj . consume (consumed) ; let status = ret ? ; if read > 0 || eof || status == Status :: StreamEnd { if read == 0 && status != Status :: StreamEnd { return Err (io :: Error :: new (io :: ErrorKind :: UnexpectedEof , "premature eof" ,)) ; } return Ok (read) ; } if consumed == 0 { return Err (io :: Error :: new (io :: ErrorKind :: InvalidData , "corrupt xz stream" ,)) ; } } } }
};
}
