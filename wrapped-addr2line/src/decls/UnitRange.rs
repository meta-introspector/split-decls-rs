macro_rules! UnitRange {
    () => {
        pub (crate) struct UnitRange { unit_id : usize , min_begin : u64 , range : gimli :: Range , }
    };
}

UnitRange!()