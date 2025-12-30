// Generated macro for ArgScopeStack (struct)
macro_rules! Depcrate_astArgScopeStack {
() => {
// Module: crate::ast
// Provides: {"ArgScopeStack"}
// Dependencies: {}
# [doc = " An `ArgScopeStack` represents the current function and template demangling"] # [doc = " scope we are within. As we enter new demangling scopes, we construct new"] # [doc = " `ArgScopeStack`s whose `prev` references point back to the old ones. These"] # [doc = " `ArgScopeStack`s are kept on the native stack, and as functions return, they"] # [doc = " go out of scope and we use the previous `ArgScopeStack`s again."] # [derive (Copy , Clone , Debug)] pub struct ArgScopeStack < 'prev , 'subs > where 'subs : 'prev , { item : & 'subs dyn ArgScope < 'subs , 'subs > , in_arg : Option < (usize , & 'subs TemplateArgs) > , prev : Option < & 'prev ArgScopeStack < 'prev , 'subs > > , }
};
}
