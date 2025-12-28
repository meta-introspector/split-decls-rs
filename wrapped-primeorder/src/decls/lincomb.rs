macro_rules! deps {
    () => {
        LookupTable!();
        PrimeCurveParams!();
        ProjectivePoint!();
    };
}

macro_rules! lincomb {
    () => {
        deps!();
        fn lincomb < C : PrimeCurveParams > (ks : & mut [ByteArray < C :: Uint >] , pcs : & mut [LookupTable < C >] ,) -> ProjectivePoint < C > { let mut q = ProjectivePoint :: IDENTITY ; let mut pos = (< Scalar < C > as PrimeField > :: NUM_BITS . div_ceil (8) * 8) as usize - 4 ; loop { for (k , pc) in ks . iter () . zip (pcs . iter ()) { let slot = (k [pos >> 3] >> (pos & 7)) & 0xf ; q = q . add (& pc . select (slot)) ; } if pos == 0 { break ; } q = Double :: double (& Double :: double (& Double :: double (& Double :: double (& q)))) ; pos -= 4 ; } q }
    };
}

lincomb!();