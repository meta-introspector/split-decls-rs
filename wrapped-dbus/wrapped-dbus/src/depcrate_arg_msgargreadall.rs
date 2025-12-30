// Generated macro for ReadAll (trait)
macro_rules! Depcrate_arg_msgargReadAll {
() => {
// Module: crate::arg::msgarg
// Provides: {"ReadAll"}
// Dependencies: {}
# [doc = " Helper trait to read all arguments from a message."] pub trait ReadAll : Sized { # [doc = " Performs the read operation."] fn read (i : & mut Iter) -> Result < Self , TypeMismatchError > ; }
};
}
