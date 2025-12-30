// Generated macro for NameConstraints (struct)
macro_rules! Depcrate_extensionsNameConstraints {
() => {
// Module: crate::extensions
// Provides: {"NameConstraints"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write)] pub struct NameConstraints < 'a , Op : Asn1Operation > { # [implicit (0)] pub permitted_subtrees : Option < SequenceOfSubtrees < 'a , Op > > , # [implicit (1)] pub excluded_subtrees : Option < SequenceOfSubtrees < 'a , Op > > , }
};
}
