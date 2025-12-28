macro_rules! deps {
    () => {
        PrimeCurveParams!();
    };
}

macro_rules! ProjectivePoint {
    () => {
        deps!();
        # [doc = " Point on a Weierstrass curve in projective coordinates."] # [derive (Clone , Copy , Debug)] pub struct ProjectivePoint < C : PrimeCurveParams > { pub (crate) x : C :: FieldElement , pub (crate) y : C :: FieldElement , pub (crate) z : C :: FieldElement , }
    };
}

ProjectivePoint!();