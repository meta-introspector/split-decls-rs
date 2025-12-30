// Generated macro for MissingLifetime (struct)
macro_rules! Depcrate_diagnosticsMissingLifetime {
() => {
// Module: crate::diagnostics
// Provides: {"MissingLifetime"}
// Dependencies: {}
# [derive (Debug)] pub struct MissingLifetime { # [doc = " Points at the name if there are no generics."] pub generics_or_segment : InFile < AstPtr < Either < ast :: GenericArgList , ast :: NameRef > > > , pub expected : u32 , pub def : GenericDef , }
};
}
