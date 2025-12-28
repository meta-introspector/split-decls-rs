macro_rules! deps {
    () => {
        PossibleValue!();
        BoolValueParser!();
    };
}

macro_rules! impl_300 {
    () => {
        deps!();
        impl BoolValueParser { # [doc = " Implementation for [`ValueParser::bool`]"] pub fn new () -> Self { Self { } } fn possible_values () -> impl Iterator < Item = crate :: builder :: PossibleValue > { ["true" , "false"] . iter () . copied () . map (crate :: builder :: PossibleValue :: new) } }
    };
}

impl_300!();