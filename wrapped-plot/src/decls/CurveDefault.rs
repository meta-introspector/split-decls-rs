macro_rules! deps {
    () => {
        Default!();
        Curve!();
    };
}

macro_rules! CurveDefault {
    () => {
        deps!();
        # [doc = " Curve variant of Default"] trait CurveDefault < S > { # [doc = " Creates `curve::Properties` with default configuration"] fn default (s : S) -> Self ; }
    };
}

CurveDefault!()