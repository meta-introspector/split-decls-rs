// Generated macro for impl_281 (impl)
macro_rules! Depcrateimpl_281 {
() => {
// Module: crate
// Provides: {"impl_281"}
// Dependencies: {}
impl < P : MlDsaParams > KeyPair < P > { # [doc = " The signing key of the key pair"] pub fn signing_key (& self) -> & SigningKey < P > { & self . signing_key } # [doc = " The verifying key of the key pair"] pub fn verifying_key (& self) -> & VerifyingKey < P > { & self . verifying_key } # [doc = " Serialize the [`Seed`] value: 32-bytes which can be used to reconstruct the"] # [doc = " [`KeyPair`]."] # [doc = ""] # [doc = " # ⚠\u{fe0f} Warning!"] # [doc = ""] # [doc = " This value is key material. Please treat it with care."] # [inline] pub fn to_seed (& self) -> Seed { self . seed } }
};
}
