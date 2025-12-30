// Generated macro for impl_73 (impl)
macro_rules! Depcrate_file_testsimpl_73 {
() => {
// Module: crate::file::tests
// Provides: {"impl_73"}
// Dependencies: {}
impl From < (Range < u32 > , ObjectId) > for UnblamedHunk { fn from (value : (Range < u32 > , ObjectId)) -> Self { let (range_in_blamed_file , suspect) = value ; let range_in_destination = range_in_blamed_file . clone () ; (range_in_blamed_file , suspect , range_in_destination) . into () } }
};
}
