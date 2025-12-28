macro_rules! deps {
    () => {
        InterpCx!();
        Machine!();
    };
}

macro_rules! impl_208 {
    () => {
        deps!();
        impl < 'tcx , M > layout :: HasTyCtxt < 'tcx > for InterpCx < 'tcx , M > where M : Machine < 'tcx > , { # [inline] fn tcx (& self) -> TyCtxt < 'tcx > { * self . tcx } }
    };
}

impl_208!()