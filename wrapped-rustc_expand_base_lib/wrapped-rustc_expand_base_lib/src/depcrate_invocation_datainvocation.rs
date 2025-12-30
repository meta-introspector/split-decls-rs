// Generated macro for Invocation (struct)
macro_rules! Depcrate_invocation_dataInvocation {
() => {
// Module: crate::invocation_data
// Provides: {"Invocation"}
// Dependencies: {}
# [doc = " Represents a single macro invocation found in the AST."] # [doc = ""] # [doc = " This struct encapsulates all the necessary information about a macro call"] # [doc = " that the expansion engine needs to process it."] # [derive (Debug)] pub struct Invocation { # [doc = " The specific kind of macro invocation (bang, attribute, derive, glob delegation)."] pub kind : InvocationKind , # [doc = " The expected kind of AST fragment that this macro should expand into."] pub fragment_kind : AstFragmentKind , # [doc = " Contextual data about where and when this macro is being expanded."] pub expansion_data : ExpansionData , }
};
}
