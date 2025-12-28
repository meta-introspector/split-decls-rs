macro_rules! deps {
    () => {
        ProjectivePoint!();
        PrimeCurveParams!();
        LookupTable!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl < C > LinearCombination < [(Self , Scalar < C >)] > for ProjectivePoint < C > where C : PrimeCurveParams , FieldBytes < C > : Copy , { # [cfg (feature = "alloc")] fn lincomb (points_and_scalars : & [(Self , Scalar < C >)]) -> Self { let (mut ks , mut pcs) : (Vec < _ > , Vec < _ >) = points_and_scalars . iter () . map (| (point , scalar) | { (Into :: < C :: Uint > :: into (* scalar) . to_le_byte_array () , LookupTable :: new (* point) ,) }) . unzip () ; lincomb :: < C > (& mut ks , & mut pcs) } }
    };
}

impl_75!();