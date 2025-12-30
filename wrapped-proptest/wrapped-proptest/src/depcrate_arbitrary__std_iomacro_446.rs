// Generated macro for macro_446 (macro)
macro_rules! Depcrate_arbitrary__std_iomacro_446 {
() => {
// Module: crate::arbitrary::_std::io
// Provides: {"macro_446"}
// Dependencies: {}
arbitrary ! ([A : Read + Arbitrary] Take < A >, SMapped < (A , u64) , Self >, A :: Parameters ; args => static_map (arbitrary_with (product_pack ! [args , Default :: default ()]) , | (a , b) | a . take (b))) ;
};
}
