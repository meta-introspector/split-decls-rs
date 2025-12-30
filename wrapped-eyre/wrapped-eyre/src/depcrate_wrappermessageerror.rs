// Generated macro for MessageError (struct)
macro_rules! Depcrate_wrapperMessageError {
() => {
// Module: crate::wrapper
// Provides: {"MessageError"}
// Dependencies: {}
# [repr (transparent)] # [doc = " Wraps a Debug + Display type as an error."] # [doc = ""] # [doc = " Its Debug and Display impls are the same as the wrapped type."] pub (crate) struct MessageError < M > (pub (crate) M) ;
};
}
