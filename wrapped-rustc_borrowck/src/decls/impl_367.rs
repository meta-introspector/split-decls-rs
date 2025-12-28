macro_rules! deps {
    () => {
        RegionValues!();
        ToElementIndex!();
    };
}

macro_rules! impl_367 {
    () => {
        deps!();
        impl ToElementIndex for ty :: PlaceholderRegion { fn add_to_row < N : Idx > (self , values : & mut RegionValues < N > , row : N) -> bool { let index = values . placeholder_indices . lookup_index (self) ; values . placeholders . insert (row , index) } fn contained_in_row < N : Idx > (self , values : & RegionValues < N > , row : N) -> bool { let index = values . placeholder_indices . lookup_index (self) ; values . placeholders . contains (row , index) } }
    };
}

impl_367!()