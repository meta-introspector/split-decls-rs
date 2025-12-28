macro_rules! deps {
    () => {
        AnyValue!();
    };
}

macro_rules! OccurrenceValues {
    () => {
        deps!();
        # [derive (Clone , Debug)] pub struct OccurrenceValues < T > { # [allow (clippy :: type_complexity)] iter : Map < std :: vec :: IntoIter < AnyValue > , fn (AnyValue) -> T > , }
    };
}

OccurrenceValues!()