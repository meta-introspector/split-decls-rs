macro_rules! deps {
    () => {
        PrimeCurveParams!();
        LookupTable!();
        ProjectivePoint!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl < C : PrimeCurveParams > LookupTable < C > { fn new (point : ProjectivePoint < C >) -> Self { let mut pc = [ProjectivePoint :: default () ; 16] ; pc [0] = ProjectivePoint :: IDENTITY ; pc [1] = point ; for i in 2 .. 16 { pc [i] = if i % 2 == 0 { Double :: double (& pc [i / 2]) } else { pc [i - 1] . add (point) } ; } Self (pc) } fn select (& self , slot : u8) -> ProjectivePoint < C > { let mut t = ProjectivePoint :: IDENTITY ; for i in 1 .. 16 { t . conditional_assign (& self . 0 [i] , Choice :: from (((slot as usize ^ i) . wrapping_sub (1) >> 8) as u8 & 1) ,) ; } t } }
    };
}

impl_79!();