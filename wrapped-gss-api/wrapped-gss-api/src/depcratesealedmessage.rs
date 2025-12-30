// Generated macro for SealedMessage (type)
macro_rules! DepcrateSealedMessage {
() => {
// Module: crate
// Provides: {"SealedMessage"}
// Dependencies: {}
# [doc = " The `SealedMessage` type is defined in [RFC 1508 Appendix B]."] # [doc = ""] # [doc = " ```text"] # [doc = " SealedMessage ::="] # [doc = " -- as emitted by GSS_Seal and processed by GSS_Unseal"] # [doc = " -- includes internal, mechanism-defined indicator"] # [doc = " -- of whether or not encrypted"] # [doc = "         sealedUserData ANY"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 1508 Appendix B]: https://datatracker.ietf.org/doc/html/rfc1508#appendix-B"] pub type SealedMessage < 'a > = AnyRef < 'a > ;
};
}
