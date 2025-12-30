// Generated macro for signature_and_consumed (function)
macro_rules! Depcrate_parsesignature_and_consumed {
() => {
// Module: crate::parse
// Provides: {"signature_and_consumed"}
// Dependencies: {}
pub (crate) fn signature_and_consumed < 'a , E : ParserError < & 'a [u8] > + AddContext < & 'a [u8] , StrContext > > (i : & mut & 'a [u8] ,) -> ModalResult < (gix_actor :: SignatureRef < 'a > , & 'a BStr) , E > { let original = * i ; gix_actor :: signature :: decode (i) . map (| signature | { let consumed = original . len () - i . len () ; (signature , original [.. consumed] . as_bstr ()) }) }
};
}
