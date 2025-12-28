macro_rules! deps {
    () => {
        PossibleValue!();
        ValueEnum!();
        ColorChoice!();
    };
}

macro_rules! impl_674 {
    () => {
        deps!();
        impl ColorChoice { # [doc = " Report all `possible_values`"] pub fn possible_values () -> impl Iterator < Item = PossibleValue > { Self :: value_variants () . iter () . filter_map (ValueEnum :: to_possible_value) } }
    };
}

impl_674!()