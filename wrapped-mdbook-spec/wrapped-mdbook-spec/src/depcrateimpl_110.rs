// Generated macro for impl_110 (impl)
macro_rules! Depcrateimpl_110 {
() => {
// Module: crate
// Provides: {"impl_110"}
// Dependencies: {}
impl Diagnostics { fn new () -> Diagnostics { let deny_warnings = std :: env :: var ("SPEC_DENY_WARNINGS") . as_deref () == Ok ("1") ; Diagnostics { deny_warnings , count : 0 , } } # [doc = " Displays a warning or error (depending on whether warnings are denied)."] # [doc = ""] # [doc = " Usually you want the [`warn_or_err!`] macro."] fn warn_or_err (& mut self , args : fmt :: Arguments < '_ >) { if self . deny_warnings { eprintln ! ("error: {args}") ; } else { eprintln ! ("warning: {args}") ; } self . count += 1 ; } }
};
}
