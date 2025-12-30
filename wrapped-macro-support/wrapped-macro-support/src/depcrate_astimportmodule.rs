// Generated macro for ImportModule (enum)
macro_rules! Depcrate_astImportModule {
() => {
// Module: crate::ast
// Provides: {"ImportModule"}
// Dependencies: {}
# [doc = " The possible types of module to import from"] # [cfg_attr (feature = "extra-traits" , derive (Debug))] # [derive (Clone)] pub enum ImportModule { # [doc = " Import from the named module, with relative paths interpreted"] Named (String , Span) , # [doc = " Import from the named module, without interpreting paths"] RawNamed (String , Span) , # [doc = " Import from an inline JS snippet"] Inline (usize) , }
};
}
