// Generated macro for RevSpecExt (trait)
macro_rules! Depcrate_ext_rev_specRevSpecExt {
() => {
// Module: crate::ext::rev_spec
// Provides: {"RevSpecExt"}
// Dependencies: {}
# [doc = " Extensions for [revision specifications][gix_revision::Spec]."] pub trait RevSpecExt { # [doc = " Attach [`Repository`][crate::Repository] to the given rev-spec."] fn attach (self , repo : & crate :: Repository) -> crate :: revision :: Spec < '_ > ; }
};
}
