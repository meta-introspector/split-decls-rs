// Generated macro for impl_2387 (impl)
macro_rules! Depcrate_io_read_lineimpl_2387 {
() => {
// Module: crate::io::read_line
// Provides: {"impl_2387"}
// Dependencies: {}
impl < R : ? Sized > Drop for ReadLine < '_ , R > { fn drop (& mut self) { if ! self . finished { self . bytes . truncate (self . bytes . len () - self . read) ; mem :: swap (unsafe { self . buf . as_mut_vec () } , & mut self . bytes) ; } } }
};
}
