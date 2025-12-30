// Generated macro for impl_78 (impl)
macro_rules! Depcrate_writeimpl_78 {
() => {
// Module: crate::write
// Provides: {"impl_78"}
// Dependencies: {}
impl < W : Write > Write for XzEncoder < W > { # [inline] fn write (& mut self , data : & [u8]) -> io :: Result < usize > { loop { self . dump () ? ; let total_in = self . total_in () ; self . data . process_vec (data , & mut self . buf , Action :: Run) ? ; let written = (self . total_in () - total_in) as usize ; if written > 0 || data . is_empty () { return Ok (written) ; } } } # [inline] fn flush (& mut self) -> io :: Result < () > { loop { self . dump () ? ; let status = self . data . process_vec (& [] , & mut self . buf , Action :: FullFlush) ? ; if status == Status :: StreamEnd { break ; } } self . obj . as_mut () . unwrap () . flush () } }
};
}
