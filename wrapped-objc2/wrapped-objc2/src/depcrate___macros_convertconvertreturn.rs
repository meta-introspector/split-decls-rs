// Generated macro for ConvertReturn (trait)
macro_rules! Depcrate___macros_convertConvertReturn {
() => {
// Module: crate::__macros::convert
// Provides: {"ConvertReturn"}
// Dependencies: {}
# [doc = " Same as [`ConvertArgument`], but for return types."] # [doc = ""] # [doc = " See `RetainSemantics` for more details."] pub trait ConvertReturn < MethodFamily > : return_private :: Sealed { type Inner : EncodeReturn ; # [track_caller] unsafe fn convert_message_return (inner : Self :: Inner , receiver_ptr : * mut AnyObject , sel : Sel ,) -> Self ; fn convert_defined_return (self) -> Self :: Inner ; }
};
}
