// Generated macro for ReferenceExt (trait)
macro_rules! Depcrate_ext_referenceReferenceExt {
() => {
// Module: crate::ext::reference
// Provides: {"ReferenceExt"}
// Dependencies: {}
# [doc = " Extensions for [references][gix_ref::Reference]."] pub trait ReferenceExt { # [doc = " Attach [`Repository`][crate::Repository] to the given reference. It can be detached later with [`detach()]`."] fn attach (self , repo : & crate :: Repository) -> crate :: Reference < '_ > ; }
};
}
