// Generated macro for MappedLookup (struct)
macro_rules! Depcrate_lookupMappedLookup {
() => {
// Module: crate::lookup
// Provides: {"MappedLookup"}
// Dependencies: {}
pub (crate) struct MappedLookup < T , L , F > where L : LookupContinuation , F : FnOnce (L :: Output) -> T , { original : L , mutator : F , }
};
}
