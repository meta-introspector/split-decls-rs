use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Compute the hash for the HIR of the full crate.
/// This hash will then be part of the crate_hash which is stored in the metadata.
fn compute_hir_hash(
    tcx: TyCtxt<'_>,
    owners: &IndexSlice<LocalDefId, hir::MaybeOwner<'_>>,
) -> Fingerprint {
    let mut hir_body_nodes: Vec<_> = owners
        .iter_enumerated()
        .filter_map(|(def_id, info)| {
            let info = info.as_owner()?;
            let def_path_hash = tcx.hir_def_path_hash(def_id);
            Some((def_path_hash, info))
        })
        .collect();
    hir_body_nodes.sort_unstable_by_key(|bn| bn.0);
    tcx.with_stable_hashing_context(|mut hcx| {
        let mut stable_hasher = StableHasher::new();
        hir_body_nodes.hash_stable(&mut hcx, &mut stable_hasher);
        stable_hasher.finish()
    })
}
