macro_rules! deps {
    () => {
        MethodResolutionUnstableFeatures!();
    };
}

macro_rules! impl_568 {
    () => {
        deps!();
        impl MethodResolutionUnstableFeatures { pub fn from_def_map (def_map : & DefMap) -> Self { Self { arbitrary_self_types : def_map . is_unstable_feature_enabled (& sym :: arbitrary_self_types) , arbitrary_self_types_pointers : def_map . is_unstable_feature_enabled (& sym :: arbitrary_self_types_pointers) , supertrait_item_shadowing : def_map . is_unstable_feature_enabled (& sym :: supertrait_item_shadowing) , } } }
    };
}

impl_568!()