macro_rules! deps {
    () => {
        LateContext!();
    };
}

macro_rules! impl_131 {
    () => {
        deps!();
        impl < 'tcx > ty :: layout :: HasTyCtxt < 'tcx > for LateContext < 'tcx > { # [inline] fn tcx (& self) -> TyCtxt < 'tcx > { self . tcx } }
    };
}

impl_131!();