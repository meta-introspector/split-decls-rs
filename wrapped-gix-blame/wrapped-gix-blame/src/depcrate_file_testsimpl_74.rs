// Generated macro for impl_74 (impl)
macro_rules! Depcrate_file_testsimpl_74 {
() => {
// Module: crate::file::tests
// Provides: {"impl_74"}
// Dependencies: {}
impl From < (Range < u32 > , ObjectId , Range < u32 >) > for UnblamedHunk { fn from (value : (Range < u32 > , ObjectId , Range < u32 >)) -> Self { let (range_in_blamed_file , suspect , range_in_destination) = value ; assert ! (range_in_blamed_file . end > range_in_blamed_file . start , "{range_in_blamed_file:?}") ; assert ! (range_in_destination . end > range_in_destination . start , "{range_in_destination:?}") ; assert_eq ! (range_in_blamed_file . len () , range_in_destination . len ()) ; UnblamedHunk { range_in_blamed_file , suspects : [(suspect , range_in_destination)] . into () , source_file_name : None , } } }
};
}
