// Generated macro for impl_111 (impl)
macro_rules! Depcrate_lookupimpl_111 {
() => {
// Module: crate::lookup
// Provides: {"impl_111"}
// Dependencies: {}
impl < T , R , F > LookupContinuation for SimpleLookup < T , R , F > where F : FnOnce (Option < Arc < gimli :: Dwarf < R > > >) -> T , R : gimli :: Reader , { type Output = T ; type Buf = R ; fn resume (self , v : Option < Arc < gimli :: Dwarf < Self :: Buf > > >) -> LookupResult < Self > { LookupResult :: Output ((self . f) (v)) } }
};
}
