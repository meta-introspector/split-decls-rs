// Generated macro for impl_114 (impl)
macro_rules! Depcrate_impls_core__cellimpl_114 {
() => {
// Module: crate::impls::core_::cell
// Provides: {"impl_114"}
// Dependencies: {}
impl < T > Format for core :: cell :: Cell < T > where T : Format + Copy , { fn format (& self , fmt : Formatter) { crate :: write ! (fmt , "Cell {{ value: {=?} }})" , self . get ()) } }
};
}
