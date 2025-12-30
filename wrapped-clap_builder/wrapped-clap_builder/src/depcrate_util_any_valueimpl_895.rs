// Generated macro for impl_895 (impl)
macro_rules! Depcrate_util_any_valueimpl_895 {
() => {
// Module: crate::util::any_value
// Provides: {"impl_895"}
// Dependencies: {}
impl AnyValueId { pub (crate) fn of < A : ? Sized + 'static > () -> Self { Self { type_id : std :: any :: TypeId :: of :: < A > () , # [cfg (debug_assertions)] type_name : std :: any :: type_name :: < A > () , } } }
};
}
