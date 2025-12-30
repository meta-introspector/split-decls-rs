// Generated macro for GeneralSubtree (struct)
macro_rules! Depcrate_extensionsGeneralSubtree {
() => {
// Module: crate::extensions
// Provides: {"GeneralSubtree"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write)] pub struct GeneralSubtree < 'a > { pub base : name :: GeneralName < 'a > , # [implicit (0)] # [default (0u64)] pub minimum : u64 , # [implicit (1)] pub maximum : Option < u64 > , }
};
}
