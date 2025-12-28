macro_rules! deps {
    () => {
        Gen!();
        Arbitrary!();
    };
}

macro_rules! unsigned_non_zero_arbitrary {
    () => {
        deps!();
        macro_rules ! unsigned_non_zero_arbitrary { ($ ($ ty : tt => $ inner : tt) ,*) => { $ (impl Arbitrary for $ ty { fn arbitrary (g : & mut Gen) -> $ ty { let mut v = $ inner :: arbitrary (g) ; if v == 0 { v += 1 ; } $ ty :: new (v) . expect ("non-zero value construction failed") } fn shrink (& self) -> Box < dyn Iterator < Item = $ ty >> { unsigned_non_zero_shrinker ! ($ inner) ; Box :: new (shrinker :: UnsignedNonZeroShrinker :: new (self . get ()) . map ($ ty :: new) . map (Option :: unwrap)) } }) * } }
    };
}

unsigned_non_zero_arbitrary!()