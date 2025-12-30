// Generated macro for macro_444 (macro)
macro_rules! Depcrate_arbitrary__std_iomacro_444 {
() => {
// Module: crate::arbitrary::_std::io
// Provides: {"macro_444"}
// Dependencies: {}
arbitrary ! ([A : BufRead + Arbitrary] Split < A >, SMapped < (A , u8) , Self >, A :: Parameters ; args => static_map (arbitrary_with (product_pack ! [args , Default :: default ()]) , | (a , b) | a . split (b))) ;
};
}
