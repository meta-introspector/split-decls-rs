// Generated macro for drop_pkt_on_err (function)
macro_rules! Depcratedrop_pkt_on_err {
() => {
// Module: crate
// Provides: {"drop_pkt_on_err"}
// Dependencies: {}
# [doc = " Maps an `Error` to `Error::Done`, or itself."] # [doc = ""] # [doc = " When a received packet that hasn't yet been authenticated triggers a failure"] # [doc = " it should, in most cases, be ignored, instead of raising a connection error,"] # [doc = " to avoid potential man-in-the-middle and man-on-the-side attacks."] # [doc = ""] # [doc = " However, if no other packet was previously received, the connection should"] # [doc = " indeed be closed as the received packet might just be network background"] # [doc = " noise, and it shouldn't keep resources occupied indefinitely."] # [doc = ""] # [doc = " This function maps an error to `Error::Done` to ignore a packet failure"] # [doc = " without aborting the connection, except when no other packet was previously"] # [doc = " received, in which case the error itself is returned, but only on the"] # [doc = " server-side as the client will already have armed the idle timer."] # [doc = ""] # [doc = " This must only be used for errors preceding packet authentication. Failures"] # [doc = " happening after a packet has been authenticated should still cause the"] # [doc = " connection to be aborted."] fn drop_pkt_on_err (e : Error , recv_count : usize , is_server : bool , trace_id : & str ,) -> Error { if is_server && recv_count == 0 { return e ; } trace ! ("{trace_id} dropped invalid packet") ; Error :: Done }
};
}
