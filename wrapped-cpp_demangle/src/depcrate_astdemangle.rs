// Generated macro for Demangle (trait)
macro_rules! Depcrate_astDemangle {
() => {
// Module: crate::ast
// Provides: {"Demangle"}
// Dependencies: {}
# [doc = " Any AST node that can be printed in a demangled form."] # [doc (hidden)] pub trait Demangle < 'subs , W > : fmt :: Debug where W : 'subs + DemangleWrite , { # [doc = " Write the demangled form of this AST node to the given context."] fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result ; }
};
}
