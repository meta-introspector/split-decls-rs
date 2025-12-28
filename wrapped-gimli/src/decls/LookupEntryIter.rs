macro_rules! deps {
    () => {
        Reader!();
        LookupParser!();
    };
}

macro_rules! LookupEntryIter {
    () => {
        deps!();
        # [derive (Clone , Debug)] pub struct LookupEntryIter < R , Parser > where R : Reader , Parser : LookupParser < R > , { current_set : Option < (R , Parser :: Header) > , remaining_input : R , }
    };
}

LookupEntryIter!();