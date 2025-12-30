// Generated macro for impl_879 (impl)
macro_rules! Depcrate_util_primitivesimpl_879 {
() => {
// Module: crate::util::primitives
// Provides: {"impl_879"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < T > core :: ops :: Index < SmallIndex > for Vec < T > { type Output = T ; # [inline] fn index (& self , index : SmallIndex) -> & T { & self [index . as_usize ()] } }
};
}
