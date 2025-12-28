macro_rules! deps {
    () => {
        RegionValues!();
        ToElementIndex!();
    };
}

macro_rules! impl_365 {
    () => {
        deps!();
        impl ToElementIndex for Location { fn add_to_row < N : Idx > (self , values : & mut RegionValues < N > , row : N) -> bool { let index = values . location_map . point_from_location (self) ; values . points . insert (row , index) } fn contained_in_row < N : Idx > (self , values : & RegionValues < N > , row : N) -> bool { let index = values . location_map . point_from_location (self) ; values . points . contains (row , index) } }
    };
}

impl_365!()