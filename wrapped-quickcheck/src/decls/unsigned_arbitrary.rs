macro_rules! deps {
    () => {
        Gen!();
        Arbitrary!();
    };
}

macro_rules! unsigned_arbitrary {
    () => {
        deps!();
        macro_rules ! unsigned_arbitrary { ($ ($ ty : tt) ,*) => { $ (impl Arbitrary for $ ty { fn arbitrary (g : & mut Gen) -> $ ty { match g . random_range (0 .. 10) { 0 => * g . choose (unsigned_problem_values ! ($ ty)) . unwrap () , _ => g . random () } } fn shrink (& self) -> Box < dyn Iterator < Item =$ ty >> { unsigned_shrinker ! ($ ty) ; shrinker :: UnsignedShrinker :: new (* self) } }) * } }
    };
}

unsigned_arbitrary!();