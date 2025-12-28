macro_rules! impl_254 {
    () => {
        impl From < PoloniusRegionVid > for RegionVid { fn from (value : PoloniusRegionVid) -> Self { Self :: from_usize (value . as_usize ()) } }
    };
}

impl_254!();