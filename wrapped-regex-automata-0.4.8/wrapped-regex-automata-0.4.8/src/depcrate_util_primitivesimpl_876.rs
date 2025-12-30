// Generated macro for impl_876 (impl)
macro_rules! Depcrate_util_primitivesimpl_876 {
() => {
// Module: crate::util::primitives
// Provides: {"impl_876"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < T > core :: ops :: Index < SmallIndex > for Vec < T > { type Output = T ; # [inline] fn index (& self , index : SmallIndex) -> & T { & self [index . as_usize ()] } }
};
}
