// Generated macro for ProfessionInfo (struct)
macro_rules! Depcrate_extensionsProfessionInfo {
() => {
// Module: crate::extensions
// Provides: {"ProfessionInfo"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write)] pub struct ProfessionInfo < 'a , Op : Asn1Operation > { # [explicit (0)] pub naming_authority : Option < NamingAuthority < 'a > > , pub profession_items : SequenceOfDisplayTexts < 'a , Op > , pub profession_oids : Option < SequenceOfObjectIdentifiers < 'a , Op > > , pub registration_number : Option < asn1 :: PrintableString < 'a > > , pub add_profession_info : Option < & 'a [u8] > , }
};
}
