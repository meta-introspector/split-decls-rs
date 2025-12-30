// Generated macro for SimpleLookup (struct)
macro_rules! Depcrate_lookupSimpleLookup {
() => {
// Module: crate::lookup
// Provides: {"SimpleLookup"}
// Dependencies: {}
pub (crate) struct SimpleLookup < T , R , F > where F : FnOnce (Option < Arc < gimli :: Dwarf < R > > >) -> T , R : gimli :: Reader , { f : F , phantom : PhantomData < (T , R) > , }
};
}
