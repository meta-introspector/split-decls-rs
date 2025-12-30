// Generated macro for EncKeyWithID (struct)
macro_rules! Depcrate_popEncKeyWithID {
() => {
// Module: crate::pop
// Provides: {"EncKeyWithID"}
// Dependencies: {}
# [doc = " The `SubsequentMessage` type is defined in [RFC 4211 Section 4.2.1]."] # [doc = ""] # [doc = " ```text"] # [doc = "   EncKeyWithID ::= SEQUENCE {"] # [doc = "       privateKey           PrivateKeyInfo,"] # [doc = "       identifier CHOICE {"] # [doc = "           string             UTF8String,"] # [doc = "           generalName        GeneralName"] # [doc = "       } OPTIONAL"] # [doc = "   }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 4211 Section 4.2.1]: https://www.rfc-editor.org/rfc/rfc4211#section-4.2.1"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct EncKeyWithID < 'a > { pub priv_key : PrivateKeyInfo , pub identifier : Option < EncKeyWithIdChoice < 'a > > , }
};
}
