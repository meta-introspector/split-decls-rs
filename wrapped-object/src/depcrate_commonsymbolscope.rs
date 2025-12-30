// Generated macro for SymbolScope (enum)
macro_rules! Depcrate_commonSymbolScope {
() => {
// Module: crate::common
// Provides: {"SymbolScope"}
// Dependencies: {}
# [doc = " A symbol scope."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub enum SymbolScope { # [doc = " Unknown scope."] Unknown , # [doc = " Symbol is visible to the compilation unit."] Compilation , # [doc = " Symbol is visible to the static linkage unit."] Linkage , # [doc = " Symbol is visible to dynamically linked objects."] Dynamic , }
};
}
