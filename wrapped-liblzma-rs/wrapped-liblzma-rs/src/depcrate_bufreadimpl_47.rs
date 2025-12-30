// Generated macro for impl_47 (impl)
macro_rules! Depcrate_bufreadimpl_47 {
() => {
// Module: crate::bufread
// Provides: {"impl_47"}
// Dependencies: {}
impl < R : BufRead > Read for XzEncoder < R > { # [inline] fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { if buf . is_empty () { return Ok (0) ; } loop { let (read , consumed , eof , ret) ; { let input = self . obj . fill_buf () ? ; eof = input . is_empty () ; let before_out = self . data . total_out () ; let before_in = self . data . total_in () ; let action = if eof { Action :: Finish } else { Action :: Run } ; ret = self . data . process (input , buf , action) ; read = (self . data . total_out () - before_out) as usize ; consumed = (self . data . total_in () - before_in) as usize ; } ; self . obj . consume (consumed) ; ret ? ; if read == 0 && ! eof { continue ; } return Ok (read) ; } } }
};
}
