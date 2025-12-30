// Generated macro for __put_error (function)
macro_rules! Depcrate__put_error {
() => {
// Module: crate
// Provides: {"__put_error"}
// Dependencies: {}
# [doc = " This is not considered part of this crate's public API. It is subject to change at any time."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `file` and `message` must be null-terminated."] # [doc (hidden)] pub unsafe fn __put_error < T > (func : Function < T > , reason : Reason < T > , file : & 'static str , line : u32 , message : Option < Cow < 'static , str > > ,) where T : Library , { put_error_inner (T :: id () , func . 0 , reason . 0 , file , line , message) }
};
}
