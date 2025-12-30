// Generated macro for impl_221 (impl)
macro_rules! Depcrate_data_input_bytes_to_entriesimpl_221 {
() => {
// Module: crate::data::input::bytes_to_entries
// Provides: {"impl_221"}
// Dependencies: {}
impl < R , W > io :: BufRead for PassThrough < R , W > where Self : io :: Read , R : io :: BufRead , W : io :: Write , { fn fill_buf (& mut self) -> io :: Result < & [u8] > { self . read . fill_buf () } fn consume (& mut self , amt : usize) { let buf = self . read . fill_buf () . expect ("never fail as we called fill-buf before and this does nothing") ; self . write . write_all (& buf [.. amt]) . expect ("a write to never fail - should be a memory buffer") ; self . read . consume (amt) ; } }
};
}
