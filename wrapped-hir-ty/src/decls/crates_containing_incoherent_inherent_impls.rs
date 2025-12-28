macro_rules! deps {
    () => {
        HirDatabase!();
    };
}

macro_rules! crates_containing_incoherent_inherent_impls {
    () => {
        deps!();
        # [salsa :: tracked (returns (ref))] fn crates_containing_incoherent_inherent_impls (db : & dyn HirDatabase) -> Box < [Crate] > { db . all_crates () . iter () . copied () . filter (| krate | krate . data (db) . origin . is_lang ()) . collect () }
    };
}

crates_containing_incoherent_inherent_impls!();