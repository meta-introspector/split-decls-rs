// Generated macro for impl_96 (impl)
macro_rules! Depcrate___macros_convertimpl_96 {
() => {
// Module: crate::__macros::convert
// Provides: {"impl_96"}
// Dependencies: {}
impl ConvertArgument for bool { type __Inner = Bool ; type __WritebackOnDrop = () ; # [inline] fn __from_defined_param (inner : Self :: __Inner) -> Self { inner . as_bool () } # [inline] unsafe fn __into_argument (self) -> (Self :: __Inner , Self :: __WritebackOnDrop) { (Bool :: new (self) , ()) } }
};
}
