macro_rules! deps {
    () => {
        CodegenCx!();
    };
}

macro_rules! impl_228 {
    () => {
        deps!();
        impl < 'tcx > ty :: layout :: HasTyCtxt < 'tcx > for CodegenCx < '_ , 'tcx > { # [inline] fn tcx (& self) -> TyCtxt < 'tcx > { self . tcx } }
    };
}

impl_228!()