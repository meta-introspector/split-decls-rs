// Generated macro for impl_157 (impl)
macro_rules! Depcrate___macros_define_class_checksimpl_157 {
() => {
// Module: crate::__macros::define_class::checks
// Provides: {"impl_157"}
// Dependencies: {}
impl < T : Message > MaybeOptionRetained for Retained < T > { type Inner = T ; # [inline] fn consumed_return (self) -> RetainedReturnValue { let ptr : * mut T = Retained :: into_raw (self) ; RetainedReturnValue (ptr . cast ()) } # [inline] fn autorelease_return (self) -> RetainedReturnValue { let ptr : * mut T = Retained :: autorelease_return (self) ; RetainedReturnValue (ptr . cast ()) } }
};
}
