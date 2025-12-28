macro_rules! deps {
    () => {
        TyVidEqKey!();
    };
}

macro_rules! impl_226 {
    () => {
        deps!();
        impl < 'tcx > From < ty :: TyVid > for TyVidEqKey < 'tcx > { # [inline] fn from (vid : ty :: TyVid) -> Self { TyVidEqKey { vid , phantom : PhantomData } } }
    };
}

impl_226!()