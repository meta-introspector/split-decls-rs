// Generated macro for SimpleHkdfExtract (type)
macro_rules! DepcrateSimpleHkdfExtract {
() => {
// Module: crate
// Provides: {"SimpleHkdfExtract"}
// Dependencies: {}
# [doc = " [`GenericHkdfExtract`] variant which uses [`SimpleHmac`] for the underlying HMAC implementation."] pub type SimpleHkdfExtract < H > = GenericHkdfExtract < SimpleHmac < H > > ;
};
}
