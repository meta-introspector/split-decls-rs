// Generated macro for negotiate_max_idle_timeout (function)
macro_rules! Depcrate_connectionnegotiate_max_idle_timeout {
() => {
// Module: crate::connection
// Provides: {"negotiate_max_idle_timeout"}
// Dependencies: {}
# [doc = " Compute the negotiated idle timeout based on local and remote max_idle_timeout transport parameters."] # [doc = ""] # [doc = " According to the definition of max_idle_timeout, a value of `0` means the timeout is disabled; see <https://www.rfc-editor.org/rfc/rfc9000#section-18.2-4.4.1.>"] # [doc = ""] # [doc = " According to the negotiation procedure, either the minimum of the timeouts or one specified is used as the negotiated value; see <https://www.rfc-editor.org/rfc/rfc9000#section-10.1-2.>"] # [doc = ""] # [doc = " Returns the negotiated idle timeout as a `Duration`, or `None` when both endpoints have opted out of idle timeout."] fn negotiate_max_idle_timeout (x : Option < VarInt > , y : Option < VarInt >) -> Option < Duration > { match (x , y) { (Some (VarInt (0)) | None , Some (VarInt (0)) | None) => None , (Some (VarInt (0)) | None , Some (y)) => Some (Duration :: from_millis (y . 0)) , (Some (x) , Some (VarInt (0)) | None) => Some (Duration :: from_millis (x . 0)) , (Some (x) , Some (y)) => Some (Duration :: from_millis (cmp :: min (x , y) . 0)) , } }
};
}
