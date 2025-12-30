// Generated macro for impl_33 (impl)
macro_rules! Depcrate_paramsimpl_33 {
() => {
// Module: crate::params
// Provides: {"impl_33"}
// Dependencies: {}
impl < 'a > RsaOaepParams < 'a > { # [doc = " Create new RsaPssParams for the provided digest and default (empty) label"] pub fn new < D > () -> Self where D : AssociatedOid , { Self :: new_with_label :: < D > (& []) } # [doc = " Create new RsaPssParams for the provided digest and specified label"] pub fn new_with_label < D > (label : & 'a impl AsRef < [u8] >) -> Self where D : AssociatedOid , { Self { hash : AlgorithmIdentifierRef { oid : D :: OID , parameters : Some (AnyRef :: NULL) , } , mask_gen : AlgorithmIdentifier { oid : OID_MGF_1 , parameters : Some (AlgorithmIdentifierRef { oid : D :: OID , parameters : Some (AnyRef :: NULL) , }) , } , p_source : pspecified_algorithm_identifier (label) , } } fn context_specific_hash (& self) -> Option < ContextSpecificRef < '_ , AlgorithmIdentifierRef < 'a > > > { if self . hash == SHA_1_AI { None } else { Some (ContextSpecificRef { tag_number : TagNumber (0) , tag_mode : TagMode :: Explicit , value : & self . hash , }) } } fn context_specific_mask_gen (& self ,) -> Option < ContextSpecificRef < '_ , AlgorithmIdentifier < AlgorithmIdentifierRef < 'a > > > > { if self . mask_gen == default_mgf1_sha1 () { None } else { Some (ContextSpecificRef { tag_number : TagNumber (1) , tag_mode : TagMode :: Explicit , value : & self . mask_gen , }) } } fn context_specific_p_source (& self ,) -> Option < ContextSpecificRef < '_ , AlgorithmIdentifierRef < 'a > > > { if self . p_source == default_pempty_string () { None } else { Some (ContextSpecificRef { tag_number : TagNumber (2) , tag_mode : TagMode :: Explicit , value : & self . p_source , }) } } }
};
}
