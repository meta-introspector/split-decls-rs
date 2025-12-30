// Generated macro for Message (struct)
macro_rules! Depcrate_multiMessage {
() => {
// Module: crate::multi
// Provides: {"Message"}
// Dependencies: {}
# [doc = " Message from the `messages` function of a multi handle."] # [doc = ""] # [doc = " Currently only indicates whether a transfer is done."] pub struct Message < 'multi > { ptr : * mut curl_sys :: CURLMsg , _multi : & 'multi Multi , }
};
}
