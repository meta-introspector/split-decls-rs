// Generated macro for impl_170 (impl)
macro_rules! Depcrate_ecdhimpl_170 {
() => {
// Module: crate::ecdh
// Provides: {"impl_170"}
// Dependencies: {}
impl < C : Curve > From < FieldBytes < C > > for SharedSecret < C > { # [doc = " NOTE: this impl is intended to be used by curve implementations to"] # [doc = " instantiate a [`SharedSecret`] value from their respective"] # [doc = " [`AffinePoint`] type."] # [doc = ""] # [doc = " Curve implementations should provide the field element representing"] # [doc = " the affine x-coordinate as `secret_bytes`."] fn from (secret_bytes : FieldBytes < C >) -> Self { Self { secret_bytes } } }
};
}
