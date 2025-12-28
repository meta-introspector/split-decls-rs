macro_rules! deps {
    () => {
        SimpleLookup!();
        SplitDwarfLoad!();
        LookupResult!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl < T , R , F > SimpleLookup < T , R , F > where F : FnOnce (Option < Arc < gimli :: Dwarf < R > > >) -> T , R : gimli :: Reader , { pub (crate) fn new_complete (t : F :: Output) -> LookupResult < SimpleLookup < T , R , F > > { LookupResult :: Output (t) } pub (crate) fn new_needs_load (load : SplitDwarfLoad < R > , f : F ,) -> LookupResult < SimpleLookup < T , R , F > > { LookupResult :: Load { load , continuation : SimpleLookup { f , phantom : PhantomData , } , } } }
    };
}

impl_68!()