// Generated macro for Admissions (struct)
macro_rules! Depcrate_extensionsAdmissions {
() => {
// Module: crate::extensions
// Provides: {"Admissions"}
// Dependencies: {}
# [derive (asn1 :: Asn1Read , asn1 :: Asn1Write)] pub struct Admissions < 'a , Op : Asn1Operation > { pub admission_authority : Option < name :: GeneralName < 'a > > , pub contents_of_admissions : Op :: SequenceOfVec < 'a , Admission < 'a , Op > > , }
};
}
