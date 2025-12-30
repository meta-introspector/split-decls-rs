// Generated macro for impl_435 (impl)
macro_rules! Depcrate___macros_writebackimpl_435 {
() => {
// Module: crate::__macros::writeback
// Provides: {"impl_435"}
// Dependencies: {}
impl < T : Message + 'static > ConvertArgument for Option < & mut Option < Retained < T > > > { type __Inner = Option < NonNull < * mut T > > ; type __WritebackOnDrop = Option < WritebackOnDropNullable < T > > ; # [inline] fn __from_defined_param (_inner : Self :: __Inner) -> Self { todo ! ("`Option<&mut Option<Retained<_>>>` is not supported in `define_class!` yet") } # [inline] unsafe fn __into_argument (self) -> (Self :: __Inner , Self :: __WritebackOnDrop) { if let Some (this) = self { let (ptr , stored) = unsafe { this . __into_argument () } ; (Some (ptr) , Some (stored)) } else { (None , None) } } }
};
}
