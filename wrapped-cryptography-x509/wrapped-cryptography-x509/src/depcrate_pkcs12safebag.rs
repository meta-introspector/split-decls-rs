// Generated macro for SafeBag (struct)
macro_rules! Depcrate_pkcs12SafeBag {
() => {
// Module: crate::pkcs12
// Provides: {"SafeBag"}
// Dependencies: {}
# [derive (asn1 :: Asn1Write)] pub struct SafeBag < 'a > { pub _bag_id : asn1 :: DefinedByMarker < asn1 :: ObjectIdentifier > , # [defined_by (_bag_id)] pub bag_value : asn1 :: Explicit < BagValue < 'a > , 0 > , pub attributes : Option < asn1 :: SetOfWriter < 'a , Attribute < 'a > , Vec < Attribute < 'a > > > > , }
};
}
