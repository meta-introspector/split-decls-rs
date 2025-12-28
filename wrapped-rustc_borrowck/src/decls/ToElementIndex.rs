macro_rules! deps {
    () => {
        RegionValues!();
    };
}

macro_rules! ToElementIndex {
    () => {
        deps!();
        pub (crate) trait ToElementIndex : Debug + Copy { fn add_to_row < N : Idx > (self , values : & mut RegionValues < N > , row : N) -> bool ; fn contained_in_row < N : Idx > (self , values : & RegionValues < N > , row : N) -> bool ; }
    };
}

ToElementIndex!()