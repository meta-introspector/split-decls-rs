macro_rules! deps {
    () => {
        Reverse!();
        ConstraintGraphDirection!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl ConstraintGraphDirection for Reverse { fn start_region (_sup : RegionVid , sub : RegionVid) -> RegionVid { sub } fn end_region (sup : RegionVid , _sub : RegionVid) -> RegionVid { sup } fn is_normal () -> bool { false } }
    };
}

impl_24!()