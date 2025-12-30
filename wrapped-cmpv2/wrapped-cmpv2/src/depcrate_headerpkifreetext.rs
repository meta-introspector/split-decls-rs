// Generated macro for PkiFreeText (type)
macro_rules! Depcrate_headerPkiFreeText {
() => {
// Module: crate::header
// Provides: {"PkiFreeText"}
// Dependencies: {}
# [doc = " The `PKIFreeText` type is defined in [RFC 4210 Section 5.1.1]"] # [doc = ""] # [doc = " ```text"] # [doc = "  PKIFreeText ::= SEQUENCE SIZE (1..MAX) OF UTF8String"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 4210 Section 5.1.1]: https://www.rfc-editor.org/rfc/rfc4210#section-5.1.1"] pub type PkiFreeText < 'a > = Vec < Utf8StringRef < 'a > > ;
};
}
