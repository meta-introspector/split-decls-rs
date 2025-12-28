macro_rules! deps {
    () => {
        OwnerInfo!();
    };
}

macro_rules! MaybeOwner {
    () => {
        deps!();
        # [derive (Copy , Clone , Debug , HashStable_Generic)] pub enum MaybeOwner < 'tcx > { Owner (& 'tcx OwnerInfo < 'tcx >) , NonOwner (HirId) , # [doc = " Used as a placeholder for unused LocalDefId."] Phantom , }
    };
}

MaybeOwner!();