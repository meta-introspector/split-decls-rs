macro_rules! deps {
    () => {
        LoopingLookup!();
        LookupContinuation!();
        LookupResult!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        impl < T , L , F > LookupContinuation for LoopingLookup < T , L , F > where L : LookupContinuation , F : FnMut (L :: Output) -> ControlFlow < T , LookupResult < L > > , { type Output = T ; type Buf = L :: Buf ; fn resume (self , v : Option < Arc < gimli :: Dwarf < Self :: Buf > > >) -> LookupResult < Self > { let r = self . continuation . resume (v) ; LoopingLookup :: new_lookup (r , self . mutator) } }
    };
}

impl_74!();