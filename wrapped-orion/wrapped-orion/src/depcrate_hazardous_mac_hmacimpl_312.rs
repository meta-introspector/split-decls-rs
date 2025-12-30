// Generated macro for impl_312 (impl)
macro_rules! Depcrate_hazardous_mac_hmacimpl_312 {
() => {
// Module: crate::hazardous::mac::hmac
// Provides: {"impl_312"}
// Dependencies: {}
impl < S : HmacHashFunction , const BLOCKSIZE : usize > core :: fmt :: Debug for Hmac < S , BLOCKSIZE > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { write ! (f , "Hmac {{ working_hasher: [***OMITTED***], opad_hasher: [***OMITTED***], ipad_hasher: [***OMITTED***], is_finalized: {:?} }}" , self . is_finalized) } }
};
}
