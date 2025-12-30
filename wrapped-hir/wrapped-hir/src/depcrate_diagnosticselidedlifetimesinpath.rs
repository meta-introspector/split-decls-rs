// Generated macro for ElidedLifetimesInPath (struct)
macro_rules! Depcrate_diagnosticsElidedLifetimesInPath {
() => {
// Module: crate::diagnostics
// Provides: {"ElidedLifetimesInPath"}
// Dependencies: {}
# [derive (Debug)] pub struct ElidedLifetimesInPath { # [doc = " Points at the name if there are no generics."] pub generics_or_segment : InFile < AstPtr < Either < ast :: GenericArgList , ast :: NameRef > > > , pub expected : u32 , pub def : GenericDef , pub hard_error : bool , }
};
}
