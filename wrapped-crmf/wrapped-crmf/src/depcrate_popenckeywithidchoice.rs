// Generated macro for EncKeyWithIdChoice (enum)
macro_rules! Depcrate_popEncKeyWithIdChoice {
() => {
// Module: crate::pop
// Provides: {"EncKeyWithIdChoice"}
// Dependencies: {}
# [doc = " The `SubsequentMessage` type defined in [RFC 4211 Section 4.2.1] features an inline CHOICE"] # [doc = " definition that is implemented as EncKeyWithIdChoice."] # [doc = ""] # [doc = " ```text"] # [doc = "       identifier CHOICE {"] # [doc = "           string             UTF8String,"] # [doc = "           generalName        GeneralName"] # [doc = "       } OPTIONAL"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 4211 Section 4.2.1]: https://www.rfc-editor.org/rfc/rfc4211#section-4.2.1"] # [derive (Clone , Debug , Eq , PartialEq)] # [allow (missing_docs)] pub enum EncKeyWithIdChoice < 'a > { String (Utf8StringRef < 'a >) , GeneralName (GeneralName) , }
};
}
