macro_rules! deps {
    () => {
        LookupContinuation!();
    };
}

macro_rules! MappedLookup {
    () => {
        deps!();
        pub (crate) struct MappedLookup < T , L , F > where L : LookupContinuation , F : FnOnce (L :: Output) -> T , { original : L , mutator : F , }
    };
}

MappedLookup!()