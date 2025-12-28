macro_rules! deps {
    () => {
        Iter!();
        RawOccurrenceValues!();
    };
}

macro_rules! RawOccurrences {
    () => {
        deps!();
        # [derive (Clone , Debug)] pub struct RawOccurrences < 'a > { # [allow (clippy :: type_complexity)] iter : Map < Iter < 'a , Vec < OsString > > , fn (& Vec < OsString >) -> RawOccurrenceValues < '_ > > , }
    };
}

RawOccurrences!();