// Generated macro for impl_82 (impl)
macro_rules! Depcrate_writeimpl_82 {
() => {
// Module: crate::write
// Provides: {"impl_82"}
// Dependencies: {}
impl < W : Write > Write for XzDecoder < W > { # [inline] fn write (& mut self , data : & [u8]) -> io :: Result < usize > { loop { self . dump () ? ; let before = self . total_in () ; let res = self . data . process_vec (data , & mut self . buf , Action :: Run) ? ; let written = (self . total_in () - before) as usize ; if written > 0 || data . is_empty () || res == Status :: StreamEnd { return Ok (written) ; } } } # [inline] fn flush (& mut self) -> io :: Result < () > { self . dump () ? ; self . obj . as_mut () . unwrap () . flush () } }
};
}
