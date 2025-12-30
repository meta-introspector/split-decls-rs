// Generated macro for FixedTag (trait)
macro_rules! Depcrate_tagFixedTag {
() => {
// Module: crate::tag
// Provides: {"FixedTag"}
// Dependencies: {}
# [doc = " Types which have a constant ASN.1 [`Tag`]."] # [doc = ""] # [doc = " ## Example"] # [doc = " ```"] # [doc = " use der::{FixedTag, Tag};"] # [doc = ""] # [doc = " struct MyOctetString;"] # [doc = ""] # [doc = " impl FixedTag for MyOctetString {"] # [doc = "     const TAG: Tag = Tag::OctetString;"] # [doc = " }"] # [doc = " ```"] pub trait FixedTag { # [doc = " ASN.1 tag"] const TAG : Tag ; }
};
}
