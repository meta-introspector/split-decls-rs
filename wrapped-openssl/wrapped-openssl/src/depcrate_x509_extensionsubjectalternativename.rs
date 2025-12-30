// Generated macro for SubjectAlternativeName (struct)
macro_rules! Depcrate_x509_extensionSubjectAlternativeName {
() => {
// Module: crate::x509::extension
// Provides: {"SubjectAlternativeName"}
// Dependencies: {}
# [doc = " An extension that allows additional identities to be bound to the subject"] # [doc = " of the certificate."] pub struct SubjectAlternativeName { critical : bool , items : Vec < RustGeneralName > , }
};
}
