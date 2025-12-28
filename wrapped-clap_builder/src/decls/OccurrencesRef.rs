macro_rules! deps {
    () => {
        AnyValue!();
        Iter!();
        OccurrenceValuesRef!();
    };
}

macro_rules! OccurrencesRef {
    () => {
        deps!();
        # [derive (Clone , Debug)] pub struct OccurrencesRef < 'a , T > { # [allow (clippy :: type_complexity)] iter : Map < Iter < 'a , Vec < AnyValue > > , fn (& Vec < AnyValue >) -> OccurrenceValuesRef < '_ , T > > , }
    };
}

OccurrencesRef!()