macro_rules! RegionSnapshot {
    () => {
        pub (crate) struct RegionSnapshot { any_unifications : bool , }
    };
}

RegionSnapshot!()