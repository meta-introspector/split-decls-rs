macro_rules! deps {
    () => {
        RegionVidKey!();
    };
}

macro_rules! impl_235 {
    () => {
        deps!();
        impl < 'tcx > From < ty :: RegionVid > for RegionVidKey < 'tcx > { fn from (vid : ty :: RegionVid) -> Self { RegionVidKey { vid , phantom : PhantomData } } }
    };
}

impl_235!()