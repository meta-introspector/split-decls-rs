macro_rules! deps {
    () => {
        IndexMap!();
    };
}

macro_rules! IndexSet {
    () => {
        deps!();
        # [cfg (not (feature = "std"))] pub struct IndexSet < T , S > { pub (crate) map : IndexMap < T , () , S > , }
    };
}

IndexSet!();