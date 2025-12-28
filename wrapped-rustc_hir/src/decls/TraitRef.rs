macro_rules! deps {
    () => {
        Path!();
    };
}

macro_rules! TraitRef {
    () => {
        deps!();
        # [doc = " References to traits in impls."] # [doc = ""] # [doc = " `resolve` maps each `TraitRef`'s `ref_id` to its defining trait; that's all"] # [doc = " that the `ref_id` is for. Note that `ref_id`'s value is not the `HirId` of the"] # [doc = " trait being referred to but just a unique `HirId` that serves as a key"] # [doc = " within the resolution map."] # [derive (Clone , Debug , Copy , HashStable_Generic)] pub struct TraitRef < 'hir > { pub path : & 'hir Path < 'hir > , # [stable_hasher (ignore)] pub hir_ref_id : HirId , }
    };
}

TraitRef!()