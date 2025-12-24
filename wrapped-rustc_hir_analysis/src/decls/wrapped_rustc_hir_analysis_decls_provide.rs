use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Adds query implementations to the [Providers] vtable, see [`rustc_middle::query`]
pub fn provide(providers: &mut Providers) {
    collect::provide(providers);
    coherence::provide(providers);
    check::provide(providers);
    *providers = Providers {
        check_unused_traits: check_unused::check_unused_traits,
        diagnostic_hir_wf_check: hir_wf_check::diagnostic_hir_wf_check,
        inferred_outlives_crate: outlives::inferred_outlives_crate,
        inferred_outlives_of: outlives::inferred_outlives_of,
        inherit_sig_for_delegation_item: delegation::inherit_sig_for_delegation_item,
        enforce_impl_non_lifetime_params_are_constrained: impl_wf_check::enforce_impl_non_lifetime_params_are_constrained,
        crate_variances: variance::crate_variances,
        variances_of: variance::variances_of,
        ..*providers
    };
}
