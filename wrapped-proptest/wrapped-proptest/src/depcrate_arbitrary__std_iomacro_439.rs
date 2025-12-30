// Generated macro for macro_439 (macro)
macro_rules! Depcrate_arbitrary__std_iomacro_439 {
() => {
// Module: crate::arbitrary::_std::io
// Provides: {"macro_439"}
// Dependencies: {}
arbitrary ! ([A : Read + Arbitrary , B : Read + Arbitrary] Chain < A , B >, SMapped < (A , B) , Self >, product_type ! [A :: Parameters , B :: Parameters] ; args => static_map (arbitrary_with (args) , | (a , b) | a . chain (b))) ;
};
}
