// Generated macro for impl_25 (impl)
macro_rules! Depcrate_block_apiimpl_25 {
() => {
// Module: crate::block_api
// Provides: {"impl_25"}
// Dependencies: {}
impl < D : EagerHash > KeyInit for HmacResetCore < D > { # [inline (always)] fn new (key : & Key < Self >) -> Self { Self :: new_from_slice (key . as_slice ()) . unwrap () } # [inline (always)] fn new_from_slice (key : & [u8]) -> Result < Self , InvalidLength > { let mut buf = get_der_key :: < D > (key) ; buf . iter_mut () . for_each (| b : & mut u8 | * b ^= IPAD) ; let mut digest = D :: Core :: default () ; digest . update_blocks (slice :: from_ref (& buf)) ; buf . iter_mut () . for_each (| b : & mut u8 | * b ^= IPAD ^ OPAD) ; let mut opad_digest = D :: Core :: default () ; opad_digest . update_blocks (slice :: from_ref (& buf)) ; Ok (Self { ipad_digest : digest . clone () , opad_digest , digest , }) } }
};
}
