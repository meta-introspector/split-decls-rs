// Generated macro for ArrayError (enum)
macro_rules! Depcrate_arg_messageitemArrayError {
() => {
// Module: crate::arg::messageitem
// Provides: {"ArrayError"}
// Dependencies: {}
# [derive (Debug , Copy , Clone)] # [doc = " Errors that can happen when creating a MessageItem::Array."] pub enum ArrayError { # [doc = " The array is empty."] EmptyArray , # [doc = " The array is composed of different element types."] DifferentElementTypes , # [doc = " The supplied signature is not a valid array signature"] InvalidSignature , }
};
}
