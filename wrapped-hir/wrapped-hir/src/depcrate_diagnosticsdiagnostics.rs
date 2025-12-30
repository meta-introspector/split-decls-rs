// Generated macro for diagnostics (macro)
macro_rules! Depcrate_diagnosticsdiagnostics {
() => {
// Module: crate::diagnostics
// Provides: {"diagnostics"}
// Dependencies: {}
macro_rules ! diagnostics { ($ AnyDiagnostic : ident <$ db : lifetime > -> $ ($ diag : ident $ (<$ lt : lifetime >) ?,) *) => { # [derive (Debug)] pub enum $ AnyDiagnostic <$ db > { $ ($ diag (Box <$ diag $ (<$ lt >) ?>) ,) * } $ (impl <$ db > From <$ diag $ (<$ lt >) ?> for $ AnyDiagnostic <$ db > { fn from (d : $ diag $ (<$ lt >) ?) -> $ AnyDiagnostic <$ db > { $ AnyDiagnostic ::$ diag (Box :: new (d)) } }) * } ; }
};
}
