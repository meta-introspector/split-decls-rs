// Generated macro for ipv4_mask_to_prefix (function)
macro_rules! Depcrate_maskipv4_mask_to_prefix {
() => {
// Module: crate::mask
// Provides: {"ipv4_mask_to_prefix"}
// Dependencies: {}
# [doc = " Converts a `Ipv4Addr` network mask into a prefix."] # [doc = ""] # [doc = " # Errors"] # [doc = " If the mask is invalid this will return an `PrefixLenError`."] pub fn ipv4_mask_to_prefix (mask : Ipv4Addr) -> Result < u8 , PrefixLenError > { let mask = u32 :: from (mask) ; let prefix = mask . leading_ones () ; if mask . checked_shl (prefix) . unwrap_or (0) == 0 { Ok (prefix as u8) } else { Err (PrefixLenError) } }
};
}
