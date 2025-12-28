macro_rules! deps {
    () => {
        MappedLookup!();
        LookupContinuation!();
        LookupResult!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl < T , L , F > LookupContinuation for MappedLookup < T , L , F > where L : LookupContinuation , F : FnOnce (L :: Output) -> T , { type Output = T ; type Buf = L :: Buf ; fn resume (self , v : Option < Arc < gimli :: Dwarf < Self :: Buf > > >) -> LookupResult < Self > { match self . original . resume (v) { LookupResult :: Output (t) => LookupResult :: Output ((self . mutator) (t)) , LookupResult :: Load { load , continuation } => LookupResult :: Load { load , continuation : MappedLookup { original : continuation , mutator : self . mutator , } , } , } } }
    };
}

impl_71!()