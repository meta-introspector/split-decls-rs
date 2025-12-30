// Generated macro for impl_434 (impl)
macro_rules! Depcrate___macros_writebackimpl_434 {
() => {
// Module: crate::__macros::writeback
// Provides: {"impl_434"}
// Dependencies: {}
impl < T : Message + 'static > ConvertArgument for Option < & mut Retained < T > > { type __Inner = Option < NonNull < * mut T > > ; type __WritebackOnDrop = Option < WritebackOnDrop < T > > ; # [inline] fn __from_defined_param (_inner : Self :: __Inner) -> Self { todo ! ("`Option<&mut Retained<_>>` is not supported in `define_class!` yet") } # [inline] unsafe fn __into_argument (self) -> (Self :: __Inner , Self :: __WritebackOnDrop) { if let Some (this) = self { let (ptr , helper) = unsafe { this . __into_argument () } ; (Some (ptr) , Some (helper)) } else { (None , None) } } }
};
}
