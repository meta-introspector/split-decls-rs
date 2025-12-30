// Generated macro for INLINE_DUPLICATES (static)
macro_rules! Depcrate_runtimeINLINE_DUPLICATES {
() => {
// Module: crate::runtime
// Provides: {"INLINE_DUPLICATES"}
// Dependencies: {}
static INLINE_DUPLICATES : Lazy < Mutex < BTreeSet < String > > > = Lazy :: new (| | Mutex :: new (BTreeSet :: new ())) ;
};
}
