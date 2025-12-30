// Generated macro for impl_94 (impl)
macro_rules! Depcrate___macros_convertimpl_94 {
() => {
// Module: crate::__macros::convert
// Provides: {"impl_94"}
// Dependencies: {}
impl < T : EncodeArgument > ConvertArgument for T { type __Inner = Self ; type __WritebackOnDrop = () ; # [inline] fn __from_defined_param (inner : Self :: __Inner) -> Self { inner } # [inline] unsafe fn __into_argument (self) -> (Self :: __Inner , Self :: __WritebackOnDrop) { (self , ()) } }
};
}
