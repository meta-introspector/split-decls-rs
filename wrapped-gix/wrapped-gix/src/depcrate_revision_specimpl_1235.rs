// Generated macro for impl_1235 (impl)
macro_rules! Depcrate_revision_specimpl_1235 {
() => {
// Module: crate::revision::spec
// Provides: {"impl_1235"}
// Dependencies: {}
# [doc = " Initialization"] impl < 'repo > Spec < 'repo > { # [doc = " Create a single specification which points to `id`."] pub fn from_id (id : Id < 'repo >) -> Self { Spec { inner : gix_revision :: Spec :: Include (id . inner) , path : None , repo : id . repo , first_ref : None , second_ref : None , } } }
};
}
