macro_rules! deps {
    () => {
        FalseyValueParser!();
        PossibleValue!();
    };
}

macro_rules! impl_304 {
    () => {
        deps!();
        impl FalseyValueParser { # [doc = " Parse false-like string values, everything else is `true`"] pub fn new () -> Self { Self { } } fn possible_values () -> impl Iterator < Item = crate :: builder :: PossibleValue > { crate :: util :: TRUE_LITERALS . iter () . chain (crate :: util :: FALSE_LITERALS . iter ()) . copied () . map (| l | crate :: builder :: PossibleValue :: new (l) . hide (l != "true" && l != "false")) } }
    };
}

impl_304!();