// Generated macro for ConvertArgument (trait)
macro_rules! Depcrate___macros_convertConvertArgument {
() => {
// Module: crate::__macros::convert
// Provides: {"ConvertArgument"}
// Dependencies: {}
# [doc = " Represents types that can be converted to/from an [`EncodeArgument`] type."] # [doc = ""] # [doc = " This is implemented specially for [`bool`] to allow using that as"] # [doc = " Objective-C `BOOL`, where it would otherwise not be allowed (since they"] # [doc = " are not ABI compatible)."] # [doc = ""] # [doc = " This is also done specially for `&mut Retained<_>`-like arguments, to"] # [doc = " allow using those as \"out\" / pass-by-writeback parameters."] pub trait ConvertArgument : argument_private :: Sealed { # [doc = " The inner type that this can be converted to and from."] # [doc (hidden)] type __Inner : EncodeArgument ; # [doc = " A helper type for out parameters."] # [doc = ""] # [doc = " When dropped, this will process any necessary change to the"] # [doc = " parameters."] # [doc (hidden)] type __WritebackOnDrop : Sized ; # [doc (hidden)] fn __from_defined_param (inner : Self :: __Inner) -> Self ; # [doc = " # Safety"] # [doc = ""] # [doc = " The `__WritebackOnDrop` return type must not be leaked, and the"] # [doc = " `__Inner` pointer must not be used after the `__WritebackOnDrop` has"] # [doc = " dropped."] # [doc = ""] # [doc = " NOTE: The standard way to ensure such a thing is with closures, but"] # [doc = " using those would interact poorly with backtraces of the message send,"] # [doc = " so we're forced to ensure this out of band."] # [doc (hidden)] unsafe fn __into_argument (self) -> (Self :: __Inner , Self :: __WritebackOnDrop) ; }
};
}
