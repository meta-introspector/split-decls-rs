// Generated macro for diffie_hellman (function)
macro_rules! Depcrate_ecdhdiffie_hellman {
() => {
// Module: crate::ecdh
// Provides: {"diffie_hellman"}
// Dependencies: {}
# [doc = " Low-level Elliptic Curve Diffie-Hellman (ECDH) function."] # [doc = ""] # [doc = " Whenever possible, we recommend using the high-level ECDH ephemeral API"] # [doc = " provided by [`EphemeralSecret`]."] # [doc = ""] # [doc = " However, if you are implementing a protocol which requires a static scalar"] # [doc = " value as part of an ECDH exchange, this API can be used to compute a"] # [doc = " [`SharedSecret`] from that value."] # [doc = ""] # [doc = " Note that this API operates on the low-level [`NonZeroScalar`] and"] # [doc = " [`AffinePoint`] types. If you are attempting to use the higher-level"] # [doc = " [`SecretKey`][`crate::SecretKey`] and [`PublicKey`] types, you will"] # [doc = " need to use the following conversions:"] # [doc = ""] # [doc = " ```ignore"] # [doc = " let shared_secret = elliptic_curve::ecdh::diffie_hellman("] # [doc = "     secret_key.to_nonzero_scalar(),"] # [doc = "     public_key.as_affine()"] # [doc = " );"] # [doc = " ```"] pub fn diffie_hellman < C > (secret_key : impl Borrow < NonZeroScalar < C > > , public_key : impl Borrow < AffinePoint < C > > ,) -> SharedSecret < C > where C : CurveArithmetic , { let public_point = ProjectivePoint :: < C > :: from (* public_key . borrow ()) ; let secret_point = (public_point * secret_key . borrow () . as_ref ()) . to_affine () ; SharedSecret :: new (secret_point) }
};
}
