// Generated macro for HkdfExtract (type)
macro_rules! DepcrateHkdfExtract {
() => {
// Module: crate
// Provides: {"HkdfExtract"}
// Dependencies: {}
# [doc = " [`GenericHkdfExtract`] variant which uses [`Hmac`] for the underlying HMAC implementation."] pub type HkdfExtract < H > = GenericHkdfExtract < Hmac < H > > ;
};
}
