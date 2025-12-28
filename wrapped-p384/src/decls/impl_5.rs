macro_rules! deps {
    () => {
        NistP384!();
        AffinePoint!();
        ProjectivePoint!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl CurveArithmetic for NistP384 { type AffinePoint = AffinePoint ; type ProjectivePoint = ProjectivePoint ; type Scalar = Scalar ; }
    };
}

impl_5!();