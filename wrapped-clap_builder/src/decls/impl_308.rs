macro_rules! deps {
    () => {
        BoolishValueParser!();
        PossibleValue!();
    };
}

macro_rules! impl_308 {
    () => {
        deps!();
        impl BoolishValueParser { # [doc = " Parse bool-like string values"] pub fn new () -> Self { Self { } } fn possible_values () -> impl Iterator < Item = crate :: builder :: PossibleValue > { crate :: util :: TRUE_LITERALS . iter () . chain (crate :: util :: FALSE_LITERALS . iter ()) . copied () . map (| l | crate :: builder :: PossibleValue :: new (l) . hide (l != "true" && l != "false")) } }
    };
}

impl_308!()