macro_rules! deps {
    () => {
        Builder!();
    };
}

macro_rules! impl_164 {
    () => {
        deps!();
        impl < 'tcx > ty :: layout :: HasTyCtxt < 'tcx > for Builder < '_ , '_ , 'tcx > { # [inline] fn tcx (& self) -> TyCtxt < 'tcx > { self . cx . tcx } }
    };
}

impl_164!()