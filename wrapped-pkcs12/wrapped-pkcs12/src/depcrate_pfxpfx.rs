// Generated macro for Pfx (struct)
macro_rules! Depcrate_pfxPfx {
() => {
// Module: crate::pfx
// Provides: {"Pfx"}
// Dependencies: {}
# [doc = " The `PFX` type is defined in [RFC 7292 Section 4]."] # [doc = ""] # [doc = " ```text"] # [doc = " PFX ::= SEQUENCE {"] # [doc = "     version     INTEGER {v3(3)}(v3,...),"] # [doc = "     authSafe    ContentInfo,"] # [doc = "     macData     MacData OPTIONAL"] # [doc = " }"] # [doc = ""] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 7292 Section 4]: https://www.rfc-editor.org/rfc/rfc7292#section-4"] # [derive (Debug , Sequence)] pub struct Pfx { # [doc = " the syntax version number."] pub version : Version , # [doc = " the authenticated safe"] pub auth_safe : ContentInfo , # [doc = " the message digest info"] pub mac_data : Option < MacData > , }
};
}
