// Generated macro for MacroEntity (struct)
macro_rules! Depcrate_contextMacroEntity {
() => {
// Module: crate::context
// Provides: {"MacroEntity"}
// Dependencies: {}
# [derive (Debug , Clone)] pub struct MacroEntity { # [doc = " The name and location of the macro definition."] pub (crate) id : ItemIdentifier , pub (crate) is_function_like : bool , pub (crate) macro_arguments : TokenStream , pub (crate) value : Option < Box < Expr > > , }
};
}
