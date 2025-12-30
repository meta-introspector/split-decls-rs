// Generated macro for impl_51 (impl)
macro_rules! Depcrate_simple_resetimpl_51 {
() => {
// Module: crate::simple_reset
// Provides: {"impl_51"}
// Dependencies: {}
impl < D : Digest + BlockSizeUser > KeyInit for SimpleHmacReset < D > { fn new (key : & Key < Self >) -> Self { Self :: new_from_slice (key . as_slice ()) . unwrap () } # [inline] fn new_from_slice (key : & [u8]) -> Result < Self , InvalidLength > { let der_key = get_der_key :: < D > (key) ; let mut ipad_key = der_key . clone () ; ipad_key . iter_mut () . for_each (| b : & mut u8 | * b ^= IPAD) ; let mut digest = D :: new () ; digest . update (& ipad_key) ; let mut opad_key = der_key ; opad_key . iter_mut () . for_each (| b : & mut u8 | * b ^= OPAD) ; Ok (Self { digest , opad_key , ipad_key , }) } }
};
}
