// Generated macro for AttributesDeserializer (struct)
macro_rules! Depcrate_de_attributesAttributesDeserializer {
() => {
// Module: crate::de::attributes
// Provides: {"AttributesDeserializer"}
// Dependencies: {}
# [doc = " A deserializer used to make possible to pack all attributes into a struct."] # [doc = " It is created by [`Attributes::into_map_access`] or [`Attributes::into_deserializer`]"] # [doc = " methods."] # [doc = ""] # [doc = " This deserializer always call [`Visitor::visit_map`] with self as [`MapAccess`]."] # [doc = ""] # [doc = " # Lifetime"] # [doc = ""] # [doc = " `'i` is a lifetime of the original buffer from which attributes were parsed."] # [doc = " In particular, when reader was created from a string, this is lifetime of the"] # [doc = " string."] # [derive (Debug , Clone)] pub struct AttributesDeserializer < 'i > { iter : Attributes < 'i > , # [doc = " The value of the attribute, read in last call to `next_key_seed`."] value : Option < Cow < 'i , [u8] > > , # [doc = " This prefix will be stripped from struct fields before match against attribute name."] prefix : & 'static str , # [doc = " Buffer to store attribute name as a field name exposed to serde consumers."] # [doc = " Keeped in the serializer to avoid many small allocations"] key_buf : String , }
};
}
