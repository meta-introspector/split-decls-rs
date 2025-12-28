macro_rules! deps {
    () => {
        AffinePoint!();
        PrimeCurveParams!();
        ProjectivePoint!();
    };
}

macro_rules! batch_normalize_generic {
    () => {
        deps!();
        # [doc = " Generic implementation of batch normalization."] fn batch_normalize_generic < C , P , Z , I , O > (points : & P , mut zs : Z , out : & mut O) where C : PrimeCurveParams , C :: FieldElement : BatchInvert < Z , Output = CtOption < I > > , C :: ProjectivePoint : Double , P : AsRef < [ProjectivePoint < C >] > + ? Sized , Z : AsMut < [C :: FieldElement] > , I : AsRef < [C :: FieldElement] > , O : AsMut < [AffinePoint < C >] > + ? Sized , { let points = points . as_ref () ; let out = out . as_mut () ; for i in 0 .. points . len () { zs . as_mut () [i] . conditional_assign (& points [i] . z , ! points [i] . z . ct_eq (& C :: FieldElement :: ZERO)) ; } let zs_inverses = < C :: FieldElement as BatchInvert < Z > > :: batch_invert (zs) . expect ("all elements should be non-zero") ; for i in 0 .. out . len () { out [i] = C :: AffinePoint :: conditional_select (& points [i] . to_affine_internal (zs_inverses . as_ref () [i]) , & C :: AffinePoint :: IDENTITY , points [i] . z . ct_eq (& C :: FieldElement :: ZERO) ,) ; } }
    };
}

batch_normalize_generic!();