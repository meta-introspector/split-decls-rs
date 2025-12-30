// Generated macro for Diagnostic (struct)
macro_rules! Depcrate_bridgeDiagnostic {
() => {
// Module: crate::bridge
// Provides: {"Diagnostic"}
// Dependencies: {}
# [derive (Clone , Debug)] pub struct Diagnostic < Span > { pub level : Level , pub message : String , pub spans : Vec < Span > , pub children : Vec < Diagnostic < Span > > , }
};
}
