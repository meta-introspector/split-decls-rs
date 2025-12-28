macro_rules! deps {
    () => {
        LookupResult!();
        LoopingLookup!();
        LookupContinuation!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl < T , L , F > LoopingLookup < T , L , F > where L : LookupContinuation , F : FnMut (L :: Output) -> ControlFlow < T , LookupResult < L > > , { pub (crate) fn new_complete (t : T) -> LookupResult < Self > { LookupResult :: Output (t) } pub (crate) fn new_lookup (mut r : LookupResult < L > , mut mutator : F) -> LookupResult < Self > { loop { match r { LookupResult :: Output (l) => match mutator (l) { ControlFlow :: Break (t) => return LookupResult :: Output (t) , ControlFlow :: Continue (r2) => { r = r2 ; } } , LookupResult :: Load { load , continuation } => { return LookupResult :: Load { load , continuation : LoopingLookup { continuation , mutator , } , } ; } } } } }
    };
}

impl_73!()