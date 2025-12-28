macro_rules! TraitCandidate {
    () => {
        # [derive (Debug , Clone , HashStable_Generic)] pub struct TraitCandidate { pub def_id : DefId , pub import_ids : SmallVec < [LocalDefId ; 1] > , }
    };
}

TraitCandidate!();