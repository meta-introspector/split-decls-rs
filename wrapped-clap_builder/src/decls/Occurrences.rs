macro_rules! deps {
    () => {
        AnyValue!();
        OccurrenceValues!();
    };
}

macro_rules! Occurrences {
    () => {
        deps!();
        # [derive (Clone , Debug)] pub struct Occurrences < T > { # [allow (clippy :: type_complexity)] iter : Map < std :: vec :: IntoIter < Vec < AnyValue > > , fn (Vec < AnyValue >) -> OccurrenceValues < T > > , }
    };
}

Occurrences!();