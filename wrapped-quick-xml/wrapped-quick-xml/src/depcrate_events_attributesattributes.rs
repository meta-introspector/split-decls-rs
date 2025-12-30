// Generated macro for Attributes (struct)
macro_rules! Depcrate_events_attributesAttributes {
() => {
// Module: crate::events::attributes
// Provides: {"Attributes"}
// Dependencies: {}
# [doc = " Iterator over XML attributes."] # [doc = ""] # [doc = " Yields `Result<Attribute>`. An `Err` will be yielded if an attribute is malformed or duplicated."] # [doc = " The duplicate check can be turned off by calling [`with_checks(false)`]."] # [doc = ""] # [doc = " When [`serialize`] feature is enabled, can be converted to serde's deserializer."] # [doc = ""] # [doc = " [`with_checks(false)`]: Self::with_checks"] # [doc = " [`serialize`]: ../../index.html#serialize"] # [derive (Clone)] pub struct Attributes < 'a > { # [doc = " Slice of `BytesStart` corresponding to attributes"] bytes : & 'a [u8] , # [doc = " Iterator state, independent from the actual source of bytes"] state : IterState , # [doc = " Encoding used for `bytes`"] decoder : Decoder , }
};
}
