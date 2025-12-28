macro_rules! deps {
    () => {
        HirId!();
    };
}

macro_rules! macro_14 {
    () => {
        deps!();
        rustc_data_structures :: define_stable_id_collections ! (HirIdMap , HirIdSet , HirIdMapEntry , HirId) ;
    };
}

macro_14!()