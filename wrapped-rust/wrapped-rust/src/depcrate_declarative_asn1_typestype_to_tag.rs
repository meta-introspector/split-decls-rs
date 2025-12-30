// Generated macro for type_to_tag (function)
macro_rules! Depcrate_declarative_asn1_typestype_to_tag {
() => {
// Module: crate::declarative_asn1::types
// Provides: {"type_to_tag"}
// Dependencies: {}
pub (crate) fn type_to_tag (t : & Type , encoding : & Option < pyo3 :: Py < Encoding > >) -> asn1 :: Tag { let inner_tag = match t { Type :: Sequence (_ , _) => asn1 :: Sequence :: TAG , Type :: SequenceOf (_) => asn1 :: Sequence :: TAG , Type :: Option (t) => type_to_tag (t . get () . inner . get () , encoding) , Type :: PyBool () => bool :: TAG , Type :: PyInt () => asn1 :: BigInt :: TAG , Type :: PyBytes () => < & [u8] as SimpleAsn1Readable > :: TAG , Type :: PyStr () => asn1 :: Utf8String :: TAG , Type :: PrintableString () => asn1 :: PrintableString :: TAG , Type :: UtcTime () => asn1 :: UtcTime :: TAG , Type :: GeneralizedTime () => asn1 :: GeneralizedTime :: TAG , } ; match encoding { Some (e) => match e . get () { Encoding :: Implicit (n) => asn1 :: implicit_tag (* n , inner_tag) , Encoding :: Explicit (n) => asn1 :: explicit_tag (* n) , } , None => inner_tag , } }
};
}
