macro_rules! deps {
    () => {
        EntryIndex!();
    };
}

macro_rules! PrefixLookupResult {
    () => {
        deps!();
        # [doc = " A way to indicate if a lookup, despite successful, was ambiguous or yielded exactly"] # [doc = " one result in the particular index."] pub type PrefixLookupResult = Result < EntryIndex , () > ;
    };
}

PrefixLookupResult!();