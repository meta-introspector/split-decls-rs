macro_rules! deps {
    () => {
        Arbitrary!();
        Gen!();
    };
}

macro_rules! signed_arbitrary {
    () => {
        deps!();
        macro_rules ! signed_arbitrary { ($ ($ ty : tt) ,*) => { $ (impl Arbitrary for $ ty { fn arbitrary (g : & mut Gen) -> $ ty { match g . random_range (0 .. 10) { 0 => * g . choose (signed_problem_values ! ($ ty)) . unwrap () , _ => g . random () } } fn shrink (& self) -> Box < dyn Iterator < Item =$ ty >> { signed_shrinker ! ($ ty) ; shrinker :: SignedShrinker :: new (* self) } }) * } }
    };
}

signed_arbitrary!()