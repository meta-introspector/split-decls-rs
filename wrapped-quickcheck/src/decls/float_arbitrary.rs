macro_rules! deps {
    () => {
        Gen!();
        Arbitrary!();
    };
}

macro_rules! float_arbitrary {
    () => {
        deps!();
        macro_rules ! float_arbitrary { ($ ($ t : ty , $ shrinkable : ty) ,+) => { $ (impl Arbitrary for $ t { fn arbitrary (g : & mut Gen) -> $ t { match g . random_range (0 .. 10) { 0 => * g . choose (float_problem_values ! ($ t)) . unwrap () , _ => { let exp = g . random_range ((0.) ..<$ t >:: MAX_EXP as i16 as $ t) ; let mantissa = g . random_range ((1.) .. 2.) ; let sign = * g . choose (& [- 1. , 1.]) . unwrap () ; sign * mantissa * exp . exp2 () } } } fn shrink (& self) -> Box < dyn Iterator < Item = $ t >> { signed_shrinker ! ($ shrinkable) ; let it = shrinker :: SignedShrinker :: new (* self as $ shrinkable) ; Box :: new (it . map (| x | x as $ t)) } }) * } ; }
    };
}

float_arbitrary!();