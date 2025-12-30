// Generated macro for impl_431 (impl)
macro_rules! Depcrate___macros_writebackimpl_431 {
() => {
// Module: crate::__macros::writeback
// Provides: {"impl_431"}
// Dependencies: {}
impl < T : Message + 'static > ConvertArgument for & mut Option < Retained < T > > { type __Inner = NonNull < * mut T > ; type __WritebackOnDrop = WritebackOnDropNullable < T > ; # [inline] fn __from_defined_param (_inner : Self :: __Inner) -> Self { todo ! ("`&mut Option<Retained<_>>` is not supported in `define_class!` yet") } # [inline] unsafe fn __into_argument (self) -> (Self :: __Inner , Self :: __WritebackOnDrop) { let ptr : NonNull < Option < Retained < T > > > = NonNull :: from (self) ; let ptr : NonNull < * mut T > = ptr . cast () ; let old : * mut T = unsafe { * ptr . as_ptr () } ; (ptr , WritebackOnDropNullable { ptr , old }) } }
};
}
