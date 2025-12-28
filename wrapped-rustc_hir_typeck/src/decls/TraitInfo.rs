macro_rules! TraitInfo {
    () => {
        # [derive (Copy , Clone , PartialEq , Eq)] pub (crate) struct TraitInfo { pub def_id : DefId , }
    };
}

TraitInfo!()