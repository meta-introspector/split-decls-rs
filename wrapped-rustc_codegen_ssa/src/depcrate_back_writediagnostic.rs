// Generated macro for Diagnostic (struct)
macro_rules! Depcrate_back_writeDiagnostic {
() => {
// Module: crate::back::write
// Provides: {"Diagnostic"}
// Dependencies: {}
struct Diagnostic { level : Level , messages : Vec < (DiagMessage , Style) > , code : Option < ErrCode > , children : Vec < Subdiagnostic > , args : DiagArgMap , }
};
}
