macro_rules! deps {
    () => {
        AccessFactsExtractor!();
    };
}

macro_rules! impl_230 {
    () => {
        deps!();
        impl < 'tcx > AccessFactsExtractor < '_ , 'tcx > { fn location_to_index (& self , location : Location) -> LocationIndex { self . location_table . mid_index (location) } }
    };
}

impl_230!()