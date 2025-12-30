// Generated macro for Admission (struct)
macro_rules! Depcrate_extensionsAdmission {
() => {
// Module: crate::extensions
// Provides: {"Admission"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write)] pub struct Admission < 'a , Op : Asn1Operation + 'a > { # [explicit (0)] pub admission_authority : Option < name :: GeneralName < 'a > > , # [explicit (1)] pub naming_authority : Option < NamingAuthority < 'a > > , pub profession_infos : Op :: SequenceOfVec < 'a , ProfessionInfo < 'a , Op > > , }
};
}
