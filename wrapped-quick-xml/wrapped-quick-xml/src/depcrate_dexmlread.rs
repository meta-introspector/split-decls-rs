// Generated macro for XmlRead (trait)
macro_rules! Depcrate_deXmlRead {
() => {
// Module: crate::de
// Provides: {"XmlRead"}
// Dependencies: {}
# [doc = " Trait used by the deserializer for iterating over input. This is manually"] # [doc = " \"specialized\" for iterating over `&[u8]`."] # [doc = ""] # [doc = " You do not need to implement this trait, it is needed to abstract from"] # [doc = " [borrowing](SliceReader) and [copying](IoReader) data sources and reuse code in"] # [doc = " deserializer"] pub trait XmlRead < 'i > { # [doc = " Return an input-borrowing event."] fn next (& mut self) -> Result < PayloadEvent < 'i > , DeError > ; # [doc = " Skips until end element is found. Unlike `next()` it will not allocate"] # [doc = " when it cannot satisfy the lifetime."] fn read_to_end (& mut self , name : QName) -> Result < () , DeError > ; # [doc = " A copy of the reader's decoder used to decode strings."] fn decoder (& self) -> Decoder ; # [doc = " Checks if the `start` tag has a [`xsi:nil`] attribute. This method ignores"] # [doc = " any errors in attributes."] # [doc = ""] # [doc = " [`xsi:nil`]: https://www.w3.org/TR/xmlschema-1/#xsi_nil"] fn has_nil_attr (& self , start : & BytesStart) -> bool ; }
};
}
