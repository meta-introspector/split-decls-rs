// Generated macro for Tagged (trait)
macro_rules! Depcrate_tagTagged {
() => {
// Module: crate::tag
// Provides: {"Tagged"}
// Dependencies: {}
# [doc = " Types which have an ASN.1 [`Tag`]."] # [doc = ""] # [doc = " ## Example"] # [doc = " ```"] # [doc = " use der::{Tag, Tagged};"] # [doc = ""] # [doc = " /// Struct, which Tag depends on data"] # [doc = " struct MyOctetOrBitString(bool);"] # [doc = ""] # [doc = " impl Tagged for MyOctetOrBitString {"] # [doc = "     fn tag(&self) -> Tag {"] # [doc = "         if self.0 {"] # [doc = "             Tag::OctetString"] # [doc = "         } else {"] # [doc = "             Tag::BitString"] # [doc = "         }"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [diagnostic :: on_unimplemented (note = "Consider adding impl of `FixedTag` to `{Self}`")] pub trait Tagged { # [doc = " Get the ASN.1 tag that this type is encoded with."] fn tag (& self) -> Tag ; }
};
}
