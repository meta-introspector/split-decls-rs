// Generated macro for impl_110 (impl)
macro_rules! Depcrate_lookupimpl_110 {
() => {
// Module: crate::lookup
// Provides: {"impl_110"}
// Dependencies: {}
impl < T , R , F > SimpleLookup < T , R , F > where F : FnOnce (Option < Arc < gimli :: Dwarf < R > > >) -> T , R : gimli :: Reader , { pub (crate) fn new_complete (t : F :: Output) -> LookupResult < SimpleLookup < T , R , F > > { LookupResult :: Output (t) } pub (crate) fn new_needs_load (load : SplitDwarfLoad < R > , f : F ,) -> LookupResult < SimpleLookup < T , R , F > > { LookupResult :: Load { load , continuation : SimpleLookup { f , phantom : PhantomData , } , } } }
};
}
