// Generated macro for UnresolvedMethodCall (struct)
macro_rules! Depcrate_diagnosticsUnresolvedMethodCall {
() => {
// Module: crate::diagnostics
// Provides: {"UnresolvedMethodCall"}
// Dependencies: {}
# [derive (Debug)] pub struct UnresolvedMethodCall < 'db > { pub expr : InFile < ExprOrPatPtr > , pub receiver : Type < 'db > , pub name : Name , pub field_with_same_name : Option < Type < 'db > > , pub assoc_func_with_same_name : Option < Function > , }
};
}
