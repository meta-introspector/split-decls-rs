// Generated macro for impl_154 (impl)
macro_rules! Depcrate_impls_core__opsimpl_154 {
() => {
// Module: crate::impls::core_::ops
// Provides: {"impl_154"}
// Dependencies: {}
impl < Idx > Format for core :: ops :: RangeInclusive < Idx > where Idx : Format , { fn format (& self , fmt : Formatter) { crate :: write ! (fmt , "{}..={}" , self . start () , self . end ()) } }
};
}
