// Generated macro for GenericArgsProhibited (struct)
macro_rules! Depcrate_diagnosticsGenericArgsProhibited {
() => {
// Module: crate::diagnostics
// Provides: {"GenericArgsProhibited"}
// Dependencies: {}
# [derive (Debug)] pub struct GenericArgsProhibited { pub args : InFile < AstPtr < Either < ast :: GenericArgList , ast :: ParenthesizedArgList > > > , pub reason : GenericArgsProhibitedReason , }
};
}
