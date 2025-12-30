// Generated macro for impl_31 (impl)
macro_rules! Depcrate_cpiimpl_31 {
() => {
// Module: crate::cpi
// Provides: {"impl_31"}
// Dependencies: {}
impl ReturnData { # [doc = " Returns the program that most recently set the return data."] pub fn program_id (& self) -> & Address { & self . program_id } # [doc = " Return the data set by the program."] pub fn as_slice (& self) -> & [u8] { unsafe { from_raw_parts (self . data . as_ptr () as _ , self . size) } } }
};
}
