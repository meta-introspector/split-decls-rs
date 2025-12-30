// Generated macro for Hkdf (type)
macro_rules! DepcrateHkdf {
() => {
// Module: crate
// Provides: {"Hkdf"}
// Dependencies: {}
# [doc = " [`GenericHkdf`] variant which uses [`Hmac`] for the underlying HMAC implementation."] pub type Hkdf < H > = GenericHkdf < Hmac < H > > ;
};
}
