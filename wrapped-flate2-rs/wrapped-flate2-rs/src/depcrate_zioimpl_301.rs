// Generated macro for impl_301 (impl)
macro_rules! Depcrate_zioimpl_301 {
() => {
// Module: crate::zio
// Provides: {"impl_301"}
// Dependencies: {}
impl < W : Write , D : Ops > Write for Writer < W , D > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { self . write_with_status (buf) . map (| res | res . 0) } fn flush (& mut self) -> io :: Result < () > { self . data . run_vec (& [] , & mut self . buf , Flush :: sync ()) . map_err (Into :: into) ? ; loop { self . dump () ? ; let before = self . data . total_out () ; self . data . run_vec (& [] , & mut self . buf , Flush :: none ()) . map_err (Into :: into) ? ; if before == self . data . total_out () { break ; } } self . obj . as_mut () . unwrap () . flush () } }
};
}
