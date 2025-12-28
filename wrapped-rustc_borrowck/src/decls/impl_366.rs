macro_rules! deps {
    () => {
        RegionValues!();
        ToElementIndex!();
    };
}

macro_rules! impl_366 {
    () => {
        deps!();
        impl ToElementIndex for RegionVid { fn add_to_row < N : Idx > (self , values : & mut RegionValues < N > , row : N) -> bool { values . free_regions . insert (row , self) } fn contained_in_row < N : Idx > (self , values : & RegionValues < N > , row : N) -> bool { values . free_regions . contains (row , self) } }
    };
}

impl_366!();