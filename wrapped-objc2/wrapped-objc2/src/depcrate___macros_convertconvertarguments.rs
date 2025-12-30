// Generated macro for ConvertArguments (trait)
macro_rules! Depcrate___macros_convertConvertArguments {
() => {
// Module: crate::__macros::convert
// Provides: {"ConvertArguments"}
// Dependencies: {}
pub trait ConvertArguments { # [doc (hidden)] type __Inner : EncodeArguments ; # [doc (hidden)] type __WritebackOnDrop : Sized ; # [doc (hidden)] unsafe fn __into_arguments (self) -> (Self :: __Inner , Self :: __WritebackOnDrop) ; }
};
}
