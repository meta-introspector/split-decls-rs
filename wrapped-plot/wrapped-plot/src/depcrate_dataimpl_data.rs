// Generated macro for impl_data (macro)
macro_rules! Depcrate_dataimpl_data {
() => {
// Module: crate::data
// Provides: {"impl_data"}
// Dependencies: {}
macro_rules ! impl_data { ($ ($ ty : ty) ,+) => { $ (impl Data for $ ty { fn f64 (self) -> f64 { f64 :: cast (self) } } impl <'a > Data for &'a $ ty { fn f64 (self) -> f64 { f64 :: cast (* self) } }) + } }
};
}
