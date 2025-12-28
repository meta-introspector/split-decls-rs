macro_rules! deps {
    () => {
        ColorChoice!();
        PossibleValue!();
        ValueEnum!();
    };
}

macro_rules! impl_677 {
    () => {
        deps!();
        impl ValueEnum for ColorChoice { fn value_variants < 'a > () -> & 'a [Self] { & [Self :: Auto , Self :: Always , Self :: Never] } fn to_possible_value (& self) -> Option < PossibleValue > { Some (match self { Self :: Auto => PossibleValue :: new ("auto") , Self :: Always => PossibleValue :: new ("always") , Self :: Never => PossibleValue :: new ("never") , }) } }
    };
}

impl_677!()