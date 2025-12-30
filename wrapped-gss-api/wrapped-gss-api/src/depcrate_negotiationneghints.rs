// Generated macro for NegHints (struct)
macro_rules! Depcrate_negotiationNegHints {
() => {
// Module: crate::negotiation
// Provides: {"NegHints"}
// Dependencies: {}
# [doc = " `NegHints` as defined in [MS-SPNG Section 2.2.1]."] # [doc = ""] # [doc = " ```text"] # [doc = " NegHints ::= SEQUENCE {"] # [doc = "     hintName[0] GeneralString OPTIONAL,"] # [doc = "     hintAddress[1] OCTET STRING OPTIONAL"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " [MS-SPNG Section 2.2.1]: https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-spng/8e71cf53-e867-4b79-b5b5-38c92be3d472"] # [derive (Clone , Copy , Debug , Eq , PartialEq , Sequence)] pub struct NegHints < 'a > { # [doc = " SHOULD<5> contain the string \"not_defined_in_RFC4178@please_ignore\"."] # [asn1 (context_specific = "0" , optional = "true")] pub hint_name : Option < GeneralStringRef < 'a > > , # [doc = " Never present. MUST be omitted by the sender. Note that the encoding rules, as specified in [X690], require that this structure not be present at all, not just be zero."] # [doc = ""] # [doc = " [X690]: https://www.itu.int/rec/T-REC-X.690/"] # [asn1 (context_specific = "1" , optional = "true" , tag_mode = "IMPLICIT")] pub hint_address : Option < & 'a OctetStringRef > , }
};
}
