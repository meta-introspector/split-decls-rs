// Generated macro for impl_113 (impl)
macro_rules! Depcrate_lookupimpl_113 {
() => {
// Module: crate::lookup
// Provides: {"impl_113"}
// Dependencies: {}
impl < T , L , F > LookupContinuation for MappedLookup < T , L , F > where L : LookupContinuation , F : FnOnce (L :: Output) -> T , { type Output = T ; type Buf = L :: Buf ; fn resume (self , v : Option < Arc < gimli :: Dwarf < Self :: Buf > > >) -> LookupResult < Self > { match self . original . resume (v) { LookupResult :: Output (t) => LookupResult :: Output ((self . mutator) (t)) , LookupResult :: Load { load , continuation } => LookupResult :: Load { load , continuation : MappedLookup { original : continuation , mutator : self . mutator , } , } , } } }
};
}
