// Generated macro for Hmac (struct)
macro_rules! Depcrate_hazardous_mac_hmacHmac {
() => {
// Module: crate::hazardous::mac::hmac
// Provides: {"Hmac"}
// Dependencies: {}
# [derive (Clone)] pub (crate) struct Hmac < S : HmacHashFunction , const BLOCKSIZE : usize > { working_hasher : S , opad_hasher : S , ipad_hasher : S , is_finalized : bool , }
};
}
