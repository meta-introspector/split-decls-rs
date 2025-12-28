macro_rules! deps {
    () => {
        NistP384!();
        ProjectivePoint!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl PrimeCurveArithmetic for NistP384 { type CurveGroup = ProjectivePoint ; }
    };
}

impl_6!();