macro_rules! deps {
    () => {
        UniqueCrateData!();
        Crate!();
    };
}

macro_rules! CratesMap {
    () => {
        deps!();
        # [doc = " The mapping from [`UniqueCrateData`] to their [`Crate`] input."] # [derive (Debug , Default)] pub struct CratesMap (DashMap < UniqueCrateData , Crate , BuildHasherDefault < FxHasher > >) ;
    };
}

CratesMap!();