// Generated macro for DetachGuard (struct)
macro_rules! Depcrate_multiDetachGuard {
() => {
// Module: crate::multi
// Provides: {"DetachGuard"}
// Dependencies: {}
# [doc = " A guard struct which guarantees that `curl_multi_remove_handle` will be"] # [doc = " called on an easy handle, either manually or on drop."] struct DetachGuard { multi : Arc < RawMulti > , easy : * mut curl_sys :: CURL , }
};
}
