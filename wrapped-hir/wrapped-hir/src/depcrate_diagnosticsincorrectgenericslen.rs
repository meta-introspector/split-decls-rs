// Generated macro for IncorrectGenericsLen (struct)
macro_rules! Depcrate_diagnosticsIncorrectGenericsLen {
() => {
// Module: crate::diagnostics
// Provides: {"IncorrectGenericsLen"}
// Dependencies: {}
# [derive (Debug)] pub struct IncorrectGenericsLen { # [doc = " Points at the name if there are no generics."] pub generics_or_segment : InFile < AstPtr < Either < ast :: GenericArgList , ast :: NameRef > > > , pub kind : IncorrectGenericsLenKind , pub provided : u32 , pub expected : u32 , pub def : GenericDef , }
};
}
