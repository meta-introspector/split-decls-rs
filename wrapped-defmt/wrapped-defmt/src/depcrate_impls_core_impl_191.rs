// Generated macro for impl_191 (impl)
macro_rules! Depcrate_impls_core_impl_191 {
() => {
// Module: crate::impls::core_
// Provides: {"impl_191"}
// Dependencies: {}
impl Format for core :: time :: Duration { fn format (& self , fmt : Formatter) { crate :: write ! (fmt , "Duration {{ secs: {=u64}, nanos: {=u32} }}" , self . as_secs () , self . subsec_nanos () ,) } }
};
}
