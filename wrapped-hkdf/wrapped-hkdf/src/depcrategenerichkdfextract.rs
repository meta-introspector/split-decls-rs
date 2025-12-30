// Generated macro for GenericHkdfExtract (struct)
macro_rules! DepcrateGenericHkdfExtract {
() => {
// Module: crate
// Provides: {"GenericHkdfExtract"}
// Dependencies: {}
# [doc = " Structure representing the streaming context of an HKDF-Extract operation."] # [doc = ""] # [doc = " This type is generic over HMAC implementation. Most users should use"] # [doc = " [`HkdfExtract`] or [`SimpleHkdfExtract`] type aliases."] # [derive (Clone , Debug)] pub struct GenericHkdfExtract < H : HmacImpl > { hmac : H , }
};
}
