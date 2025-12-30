// Generated macro for SymbolMapName (struct)
macro_rules! Depcrate_readSymbolMapName {
() => {
// Module: crate::read
// Provides: {"SymbolMapName"}
// Dependencies: {}
# [doc = " The type used for entries in a [`SymbolMap`] that maps from addresses to names."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct SymbolMapName < 'data > { address : u64 , name : & 'data str , }
};
}
