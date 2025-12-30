// Generated macro for Children (struct)
macro_rules! Depcrate_diagnosticChildren {
() => {
// Module: crate::diagnostic
// Provides: {"Children"}
// Dependencies: {}
# [doc = " Iterator over the children diagnostics of a `Diagnostic`."] # [derive (Debug , Clone)] # [unstable (feature = "proc_macro_diagnostic" , issue = "54140")] pub struct Children < 'a > (std :: slice :: Iter < 'a , Diagnostic >) ;
};
}
