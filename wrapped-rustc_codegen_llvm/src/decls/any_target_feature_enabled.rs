macro_rules! deps {
    () => {
        CodegenCx!();
    };
}

macro_rules! any_target_feature_enabled {
    () => {
        deps!();
        fn any_target_feature_enabled (cx : & CodegenCx < '_ , '_ > , instance : Instance < '_ > , features : & [Symbol] ,) -> bool { let enabled = cx . tcx . asm_target_features (instance . def_id ()) ; features . iter () . any (| feat | enabled . contains (feat)) }
    };
}

any_target_feature_enabled!()