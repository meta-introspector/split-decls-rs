// Generated macro for impl_38 (impl)
macro_rules! Depcrate_simpleimpl_38 {
() => {
// Module: crate::simple
// Provides: {"impl_38"}
// Dependencies: {}
impl < D : Digest + BlockSizeUser > KeyInit for SimpleHmac < D > { fn new (key : & Key < Self >) -> Self { Self :: new_from_slice (key . as_slice ()) . unwrap () } # [inline] fn new_from_slice (key : & [u8]) -> Result < Self , InvalidLength > { let mut buf = get_der_key :: < D > (key) ; buf . iter_mut () . for_each (| b : & mut u8 | * b ^= IPAD) ; let mut digest = D :: new () ; digest . update (& buf) ; buf . iter_mut () . for_each (| b : & mut u8 | * b ^= OPAD ^ IPAD) ; Ok (Self { digest , opad_key : buf , }) } }
};
}
