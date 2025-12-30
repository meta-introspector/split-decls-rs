// Generated macro for SimpleHkdf (type)
macro_rules! DepcrateSimpleHkdf {
() => {
// Module: crate
// Provides: {"SimpleHkdf"}
// Dependencies: {}
# [doc = " [`GenericHkdf`] variant which uses [`SimpleHmac`] for the underlying HMAC implementation."] pub type SimpleHkdf < H > = GenericHkdf < SimpleHmac < H > > ;
};
}
