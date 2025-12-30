// Generated macro for SymbolMap (struct)
macro_rules! Depcrate_readSymbolMap {
() => {
// Module: crate::read
// Provides: {"SymbolMap"}
// Dependencies: {}
# [doc = " A map from addresses to symbol information."] # [doc = ""] # [doc = " The symbol information depends on the chosen entry type, such as [`SymbolMapName`]."] # [doc = ""] # [doc = " Returned by [`Object::symbol_map`]."] # [derive (Debug , Default , Clone)] pub struct SymbolMap < T : SymbolMapEntry > { symbols : Vec < T > , }
};
}
