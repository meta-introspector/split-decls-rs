// Generated macro for FriendlyName (struct)
macro_rules! Depcrate_symmetric_keyFriendlyName {
() => {
// Module: crate::symmetric_key
// Provides: {"FriendlyName"}
// Dependencies: {}
# [doc = " The `FriendlyName` type is defined in [RFC 6031 Section 3.2.6]."] # [doc = ""] # [doc = " ```text"] # [doc = "    FriendlyName ::= SEQUENCE {"] # [doc = "      friendlyName        UTF8String,"] # [doc = "      friendlyNameLangTag UTF8String OPTIONAL }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 6031 Section 3.2.6]: https://datatracker.ietf.org/doc/html/rfc6031#section-3.2.6"] # [derive (Sequence , PartialEq , Eq)] # [allow (missing_docs)] pub struct FriendlyName { pub friendly_name : String , # [asn1 (optional = "true")] pub friendly_name_lang_tag : Option < String > , }
};
}
