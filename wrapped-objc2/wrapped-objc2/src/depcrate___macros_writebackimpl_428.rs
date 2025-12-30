// Generated macro for impl_428 (impl)
macro_rules! Depcrate___macros_writebackimpl_428 {
() => {
// Module: crate::__macros::writeback
// Provides: {"impl_428"}
// Dependencies: {}
impl < T : Message + 'static > ConvertArgument for & mut Retained < T > { type __Inner = NonNull < * mut T > ; type __WritebackOnDrop = WritebackOnDrop < T > ; # [inline] fn __from_defined_param (_inner : Self :: __Inner) -> Self { todo ! ("`&mut Retained<_>` is not supported in `define_class!` yet") } # [inline] unsafe fn __into_argument (self) -> (Self :: __Inner , Self :: __WritebackOnDrop) { let ptr : NonNull < Retained < T > > = NonNull :: from (self) ; let ptr : NonNull < NonNull < T > > = ptr . cast () ; let old : NonNull < T > = unsafe { * ptr . as_ptr () } ; let ptr : NonNull < * mut T > = ptr . cast () ; (ptr , WritebackOnDrop { ptr , old }) } }
};
}
