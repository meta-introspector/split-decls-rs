macro_rules! deps {
    () => {
        Normal!();
        ConstraintGraphDirection!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl ConstraintGraphDirection for Normal { fn start_region (sup : RegionVid , _sub : RegionVid) -> RegionVid { sup } fn end_region (_sup : RegionVid , sub : RegionVid) -> RegionVid { sub } fn is_normal () -> bool { true } }
    };
}

impl_22!();