macro_rules! deps {
    () => {
        PrefilterI!();
        Pre!();
        Strategy!();
        GroupInfo!();
    };
}

macro_rules! impl_380 {
    () => {
        deps!();
        impl < P : PrefilterI > Pre < P > { fn new (pre : P) -> Arc < dyn Strategy > { let group_info = GroupInfo :: new ([[None :: < & str >]]) . unwrap () ; Arc :: new (Pre { pre , group_info }) } }
    };
}

impl_380!();