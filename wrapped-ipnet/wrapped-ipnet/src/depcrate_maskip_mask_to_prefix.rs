// Generated macro for ip_mask_to_prefix (function)
macro_rules! Depcrate_maskip_mask_to_prefix {
() => {
// Module: crate::mask
// Provides: {"ip_mask_to_prefix"}
// Dependencies: {}
# [doc = " Converts a `IpAddr` network mask into a prefix."] # [doc = ""] # [doc = " # Errors"] # [doc = " If the mask is invalid this will return an `PrefixLenError`."] pub fn ip_mask_to_prefix (mask : IpAddr) -> Result < u8 , PrefixLenError > { match mask { IpAddr :: V4 (mask) => ipv4_mask_to_prefix (mask) , IpAddr :: V6 (mask) => ipv6_mask_to_prefix (mask) , } }
};
}
