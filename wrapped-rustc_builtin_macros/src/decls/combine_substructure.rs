macro_rules! deps {
    () => {
        CombineSubstructureFunc!();
    };
}

macro_rules! combine_substructure {
    () => {
        deps!();
        pub (crate) fn combine_substructure (f : CombineSubstructureFunc < '_ > ,) -> RefCell < CombineSubstructureFunc < '_ > > { RefCell :: new (f) }
    };
}

combine_substructure!();