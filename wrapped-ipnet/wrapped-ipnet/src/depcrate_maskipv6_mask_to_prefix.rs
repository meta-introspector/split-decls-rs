// Generated macro for ipv6_mask_to_prefix (function)
macro_rules! Depcrate_maskipv6_mask_to_prefix {
() => {
// Module: crate::mask
// Provides: {"ipv6_mask_to_prefix"}
// Dependencies: {}
# [doc = " Converts a `Ipv6Addr` network mask into a prefix."] # [doc = ""] # [doc = " # Errors"] # [doc = " If the mask is invalid this will return an `PrefixLenError`."] pub fn ipv6_mask_to_prefix (mask : Ipv6Addr) -> Result < u8 , PrefixLenError > { let mask = u128 :: from (mask) ; let prefix = mask . leading_ones () ; if mask . checked_shl (prefix) . unwrap_or (0) == 0 { Ok (prefix as u8) } else { Err (PrefixLenError) } }
};
}
