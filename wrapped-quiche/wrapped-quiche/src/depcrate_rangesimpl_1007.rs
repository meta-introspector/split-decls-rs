// Generated macro for impl_1007 (impl)
macro_rules! Depcrate_rangesimpl_1007 {
() => {
// Module: crate::ranges
// Provides: {"impl_1007"}
// Dependencies: {}
impl std :: fmt :: Debug for RangeSet { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { let ranges : Vec < Range < u64 > > = self . iter () . map (| mut r | { r . end -= 1 ; r }) . collect () ; write ! (f , "{ranges:?}") } }
};
}
