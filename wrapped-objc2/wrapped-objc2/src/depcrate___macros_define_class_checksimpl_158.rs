// Generated macro for impl_158 (impl)
macro_rules! Depcrate___macros_define_class_checksimpl_158 {
() => {
// Module: crate::__macros::define_class::checks
// Provides: {"impl_158"}
// Dependencies: {}
impl < T : Message > MaybeOptionRetained for Option < Retained < T > > { type Inner = T ; # [inline] fn consumed_return (self) -> RetainedReturnValue { let ptr : * mut T = Retained :: consume_as_ptr_option (self) ; RetainedReturnValue (ptr . cast ()) } # [inline] fn autorelease_return (self) -> RetainedReturnValue { let ptr : * mut T = Retained :: autorelease_return_option (self) ; RetainedReturnValue (ptr . cast ()) } }
};
}
