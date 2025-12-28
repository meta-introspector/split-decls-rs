macro_rules! impl_253 {
    () => {
        impl From < RegionVid > for PoloniusRegionVid { fn from (value : RegionVid) -> Self { Self :: from_usize (value . as_usize ()) } }
    };
}

impl_253!();