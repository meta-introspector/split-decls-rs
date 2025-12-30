// Generated macro for link (macro)
macro_rules! Depcratelink {
() => {
// Module: crate
// Provides: {"link"}
// Dependencies: {}
# [doc = " Defines an external function to import."] # [cfg (not (windows))] # [macro_export] macro_rules ! link { ($ library : literal $ abi : literal $ ($ link_name : literal) ? fn $ ($ function : tt) *) => (extern $ abi { pub fn $ ($ function) *; }) }
};
}
