macro_rules! deps {
    () => {
        SimpleLookup!();
        LookupContinuation!();
        LookupResult!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl < T , R , F > LookupContinuation for SimpleLookup < T , R , F > where F : FnOnce (Option < Arc < gimli :: Dwarf < R > > >) -> T , R : gimli :: Reader , { type Output = T ; type Buf = R ; fn resume (self , v : Option < Arc < gimli :: Dwarf < Self :: Buf > > >) -> LookupResult < Self > { LookupResult :: Output ((self . f) (v)) } }
    };
}

impl_69!();