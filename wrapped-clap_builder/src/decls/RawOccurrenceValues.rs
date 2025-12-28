macro_rules! deps {
    () => {
        Iter!();
        OsStr!();
    };
}

macro_rules! RawOccurrenceValues {
    () => {
        deps!();
        # [derive (Clone , Debug)] pub struct RawOccurrenceValues < 'a > { # [allow (clippy :: type_complexity)] iter : Map < Iter < 'a , OsString > , fn (& OsString) -> & OsStr > , }
    };
}

RawOccurrenceValues!();