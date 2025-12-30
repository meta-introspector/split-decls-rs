// Generated macro for Attribute (struct)
macro_rules! Depcrate_events_attributesAttribute {
() => {
// Module: crate::events::attributes
// Provides: {"Attribute"}
// Dependencies: {}
# [doc = " A struct representing a key/value XML attribute."] # [doc = ""] # [doc = " Field `value` stores raw bytes, possibly containing escape-sequences. Most users will likely"] # [doc = " want to access the value using one of the [`unescape_value`] and [`decode_and_unescape_value`]"] # [doc = " functions."] # [doc = ""] # [doc = " [`unescape_value`]: Self::unescape_value"] # [doc = " [`decode_and_unescape_value`]: Self::decode_and_unescape_value"] # [derive (Clone , Eq , PartialEq)] pub struct Attribute < 'a > { # [doc = " The key to uniquely define the attribute."] # [doc = ""] # [doc = " If [`Attributes::with_checks`] is turned off, the key might not be unique."] pub key : QName < 'a > , # [doc = " The raw value of the attribute."] pub value : Cow < 'a , [u8] > , }
};
}
