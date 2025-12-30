// Generated macro for provide (function)
macro_rules! Depcrate_coherenceprovide {
() => {
// Module: crate::coherence
// Provides: {"provide"}
// Dependencies: {}
# [doc = " Adds query implementations to the [Providers] vtable, see [`rustc_middle::query`]."] pub (crate) fn provide (providers : & mut Providers) { use self :: builtin :: coerce_unsized_info ; use self :: inherent_impls :: { crate_incoherent_impls , crate_inherent_impls , crate_inherent_impls_validity_check , inherent_impls , } ; use self :: inherent_impls_overlap :: crate_inherent_impls_overlap_check ; use self :: orphan :: orphan_check_impl ; * providers = Providers { coherent_trait , crate_inherent_impls , crate_incoherent_impls , inherent_impls , crate_inherent_impls_validity_check , crate_inherent_impls_overlap_check , coerce_unsized_info , orphan_check_impl , .. * providers } ; }
};
}
