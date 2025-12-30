// Generated macro for declare_with_version (macro)
macro_rules! Depcrate_deprecated_lintsdeclare_with_version {
() => {
// Module: crate::deprecated_lints
// Provides: {"declare_with_version"}
// Dependencies: {}
macro_rules ! declare_with_version { ($ name : ident ($ name_version : ident) = [$ (# [clippy :: version = $ version : literal] $ e : expr ,) *]) => { pub static $ name : & [(& str , & str)] = & [$ ($ e) ,*] ; pub static $ name_version : & [& str] = & [$ ($ version) ,*] ; } ; }
};
}
