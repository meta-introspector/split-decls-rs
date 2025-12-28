macro_rules! deps {
    () => {
        ProjectivePoint!();
        AffinePoint!();
    };
}

macro_rules! PrimeCurveParams {
    () => {
        deps!();
        # [doc = " Parameters for elliptic curves of prime order which can be described by the"] # [doc = " short Weierstrass equation."] pub trait PrimeCurveParams : PrimeCurve + CurveArithmetic + CurveArithmetic < AffinePoint = AffinePoint < Self > > + CurveArithmetic < ProjectivePoint = ProjectivePoint < Self > > { # [doc = " Base field element type."] type FieldElement : PrimeField < Repr = FieldBytes < Self > > + Invert < Output = CtOption < Self :: FieldElement > > ; # [doc = " [Point arithmetic](point_arithmetic) implementation, might be optimized for this specific curve"] type PointArithmetic : point_arithmetic :: PointArithmetic < Self > ; # [doc = " Coefficient `a` in the curve equation."] const EQUATION_A : Self :: FieldElement ; # [doc = " Coefficient `b` in the curve equation."] const EQUATION_B : Self :: FieldElement ; # [doc = " Generator point's affine coordinates: (x, y)."] const GENERATOR : (Self :: FieldElement , Self :: FieldElement) ; }
    };
}

PrimeCurveParams!()