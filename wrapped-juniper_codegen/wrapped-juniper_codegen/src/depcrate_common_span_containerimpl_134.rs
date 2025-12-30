// Generated macro for impl_134 (impl)
macro_rules! Depcrate_common_span_containerimpl_134 {
() => {
// Module: crate::common::span_container
// Provides: {"impl_134"}
// Dependencies: {}
impl < T : Hash > Hash for SpanContainer < T > { fn hash < H > (& self , state : & mut H) where H : Hasher , { self . val . hash (state) } }
};
}
