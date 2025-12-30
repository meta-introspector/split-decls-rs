// Generated macro for impl_1389 (impl)
macro_rules! Depcrate_sampleimpl_1389 {
() => {
// Module: crate::sample
// Provides: {"impl_1389"}
// Dependencies: {}
impl < T : fmt :: Debug + Clone + 'static > statics :: MapFn < usize > for SelectMapFn < T > { type Output = T ; fn apply (& self , ix : usize) -> T { self . 0 [ix] . clone () } }
};
}
