// Generated macro for OCSPCerts (type)
macro_rules! Depcrate_ocsp_respOCSPCerts {
() => {
// Module: crate::ocsp_resp
// Provides: {"OCSPCerts"}
// Dependencies: {}
pub type OCSPCerts < 'a > = Option < common :: Asn1ReadableOrWritable < asn1 :: SequenceOf < 'a , certificate :: Certificate < 'a > > , asn1 :: SequenceOfWriter < 'a , certificate :: Certificate < 'a > , Vec < certificate :: Certificate < 'a > > > , > , > ;
};
}
