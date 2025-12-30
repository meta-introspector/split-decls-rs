// Generated macro for SymbolCollector (struct)
macro_rules! Depcrate_symbolsSymbolCollector {
() => {
// Module: crate::symbols
// Provides: {"SymbolCollector"}
// Dependencies: {}
pub struct SymbolCollector < 'a > { db : & 'a dyn HirDatabase , symbols : FxIndexSet < FileSymbol > , work : Vec < SymbolCollectorWork > , current_container_name : Option < Symbol > , collect_pub_only : bool , }
};
}
