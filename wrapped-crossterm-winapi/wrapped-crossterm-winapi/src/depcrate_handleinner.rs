// Generated macro for Inner (struct)
macro_rules! Depcrate_handleInner {
() => {
// Module: crate::handle
// Provides: {"Inner"}
// Dependencies: {}
# [doc = " Inner structure for closing a handle on Drop."] # [doc = ""] # [doc = " The second parameter indicates if the HANDLE is exclusively owned or not."] # [doc = " A non-exclusive handle can be created using for example"] # [doc = " `Handle::input_handle` or `Handle::output_handle`, which corresponds to"] # [doc = " stdin and stdout respectively."] # [derive (Debug)] struct Inner { handle : HANDLE , is_exclusive : bool , }
};
}
