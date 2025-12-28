macro_rules! deps {
    () => {
        ProjectivePoint!();
        AffinePoint!();
        NistP384!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl CurveArithmetic for NistP384 { type AffinePoint = AffinePoint ; type ProjectivePoint = ProjectivePoint ; type Scalar = Scalar ; }
    };
}

impl_5!()