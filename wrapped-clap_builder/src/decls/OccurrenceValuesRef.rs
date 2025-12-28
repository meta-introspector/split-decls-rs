macro_rules! deps {
    () => {
        Iter!();
        AnyValue!();
    };
}

macro_rules! OccurrenceValuesRef {
    () => {
        deps!();
        # [derive (Clone , Debug)] pub struct OccurrenceValuesRef < 'a , T > { # [allow (clippy :: type_complexity)] iter : Map < Iter < 'a , AnyValue > , fn (& AnyValue) -> & T > , }
    };
}

OccurrenceValuesRef!();