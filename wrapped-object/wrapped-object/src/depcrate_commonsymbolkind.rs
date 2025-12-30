// Generated macro for SymbolKind (enum)
macro_rules! Depcrate_commonSymbolKind {
() => {
// Module: crate::common
// Provides: {"SymbolKind"}
// Dependencies: {}
# [doc = " The kind of a symbol."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] # [non_exhaustive] pub enum SymbolKind { # [doc = " The symbol kind is unknown."] Unknown , # [doc = " The symbol is for executable code."] Text , # [doc = " The symbol is for a data object."] Data , # [doc = " The symbol is for a section."] Section , # [doc = " The symbol is the name of a file. It precedes symbols within that file."] File , # [doc = " The symbol is for a code label."] Label , # [doc = " The symbol is for a thread local storage entity."] Tls , }
};
}
