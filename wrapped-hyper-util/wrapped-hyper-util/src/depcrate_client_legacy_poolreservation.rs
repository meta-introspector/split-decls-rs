// Generated macro for Reservation (enum)
macro_rules! Depcrate_client_legacy_poolReservation {
() => {
// Module: crate::client::legacy::pool
// Provides: {"Reservation"}
// Dependencies: {}
# [doc = " When checking out a pooled connection, it might be that the connection"] # [doc = " only supports a single reservation, or it might be usable for many."] # [doc = ""] # [doc = " Specifically, HTTP/1 requires a unique reservation, but HTTP/2 can be"] # [doc = " used for multiple requests."] # [allow (missing_debug_implementations)] pub enum Reservation < T > { # [doc = " This connection could be used multiple times, the first one will be"] # [doc = " reinserted into the `idle` pool, and the second will be given to"] # [doc = " the `Checkout`."] # [cfg (feature = "http2")] Shared (T , T) , # [doc = " This connection requires unique access. It will be returned after"] # [doc = " use is complete."] Unique (T) , }
};
}
