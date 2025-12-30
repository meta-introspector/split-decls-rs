// Generated macro for impl_134 (impl)
macro_rules! Depcrate_mask_passwordimpl_134 {
() => {
// Module: crate::mask_password
// Provides: {"impl_134"}
// Dependencies: {}
impl < 'a , T , D > PasswordReplaced < 'a , T , D > where T : ? Sized , D : fmt :: Display , { # [doc = " Creates a new `PasswordMasked` object."] # [doc = ""] # [doc = " # Precondition"] # [doc = ""] # [doc = " The given string must be a valid IRI reference."] # [inline] # [must_use] pub (crate) fn with_replacer < S , F > (iri_ref : & 'a T , replace : F) -> Self where S : Spec , T : AsRef < RiReferenceStr < S > > , F : FnOnce (& str) -> D , { let iri_ref_asref = iri_ref . as_ref () ; let password = password_range_to_hide (iri_ref_asref) . map (move | pw_range | (pw_range . clone () , replace (& iri_ref_asref . as_str () [pw_range]))) ; Self { iri_ref , password } } }
};
}
