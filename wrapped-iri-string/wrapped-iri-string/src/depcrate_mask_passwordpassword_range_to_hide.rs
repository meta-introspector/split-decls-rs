// Generated macro for password_range_to_hide (function)
macro_rules! Depcrate_mask_passwordpassword_range_to_hide {
() => {
// Module: crate::mask_password
// Provides: {"password_range_to_hide"}
// Dependencies: {}
# [doc = " Returns the range of the password to hide."] pub (crate) fn password_range_to_hide < S : Spec > (iri : & RiReferenceStr < S >) -> Option < Range < usize > > { # [doc = " Spec-agnostic internal implementation of `password_range_to_hide`."] fn inner (iri : & str , userinfo : & str) -> Option < Range < usize > > { let authority_start = 2 + iri . find ("//") . expect ("[validity] `authority` component must be prefixed with `//`") ; let end = authority_start + userinfo . len () ; let start = authority_start + userinfo . find (':') . map_or_else (| | userinfo . len () , | v | v + 1) ; Some (start .. end) } let authority_components = AuthorityComponents :: from_iri (iri) ? ; let userinfo = authority_components . userinfo () ? ; inner (iri . as_str () , userinfo) }
};
}
