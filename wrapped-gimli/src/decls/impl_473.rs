macro_rules! deps {
    () => {
        LookupParser!();
        LookupEntryIter!();
        DebugLookup!();
        Reader!();
    };
}

macro_rules! impl_473 {
    () => {
        deps!();
        impl < R , Parser > DebugLookup < R , Parser > where R : Reader , Parser : LookupParser < R > , { pub fn items (& self) -> LookupEntryIter < R , Parser > { LookupEntryIter { current_set : None , remaining_input : self . input_buffer . clone () , } } pub fn reader (& self) -> & R { & self . input_buffer } }
    };
}

impl_473!()