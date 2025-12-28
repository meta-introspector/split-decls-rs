macro_rules! deps {
    () => {
        Obligation!();
    };
}

macro_rules! impl_313 {
    () => {
        deps!();
        impl < 'tcx , T : PartialEq > PartialEq < Obligation < 'tcx , T > > for Obligation < 'tcx , T > { # [inline] fn eq (& self , other : & Obligation < 'tcx , T >) -> bool { self . param_env == other . param_env && self . predicate == other . predicate } }
    };
}

impl_313!();