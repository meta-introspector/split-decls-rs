macro_rules! deps {
    () => {
        ConstVidKey!();
    };
}

macro_rules! impl_243 {
    () => {
        deps!();
        impl < 'tcx > From < ty :: ConstVid > for ConstVidKey < 'tcx > { fn from (vid : ty :: ConstVid) -> Self { ConstVidKey { vid , phantom : PhantomData } } }
    };
}

impl_243!()