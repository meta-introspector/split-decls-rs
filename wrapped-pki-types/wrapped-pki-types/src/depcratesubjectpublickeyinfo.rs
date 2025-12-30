// Generated macro for SubjectPublicKeyInfo (type)
macro_rules! DepcrateSubjectPublicKeyInfo {
() => {
// Module: crate
// Provides: {"SubjectPublicKeyInfo"}
// Dependencies: {}
# [doc = " A DER-encoded SubjectPublicKeyInfo (SPKI), as specified in RFC 5280."] # [deprecated (since = "1.7.0" , note = "Prefer `SubjectPublicKeyInfoDer` instead")] pub type SubjectPublicKeyInfo < 'a > = SubjectPublicKeyInfoDer < 'a > ;
};
}
