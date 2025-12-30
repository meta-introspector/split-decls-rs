// Generated macro for ArgScopeStackExt (trait)
macro_rules! Depcrate_astArgScopeStackExt {
() => {
// Module: crate::ast
// Provides: {"ArgScopeStackExt"}
// Dependencies: {}
# [doc = " When we first begin demangling, we haven't entered any function or template"] # [doc = " demangling scope and we don't have any useful `ArgScopeStack`. Therefore, we"] # [doc = " are never actually dealing with `ArgScopeStack` directly in practice, but"] # [doc = " always an `Option<ArgScopeStack>` instead. Nevertheless, we want to define"] # [doc = " useful methods on `Option<ArgScopeStack>`."] # [doc = ""] # [doc = " A custom \"extension\" trait with exactly one implementor: Rust's principled"] # [doc = " monkey patching!"] trait ArgScopeStackExt < 'prev , 'subs > : Copy { # [doc = " Push a new `ArgScope` onto this `ArgScopeStack` and return the new"] # [doc = " `ArgScopeStack` with the pushed resolver on top."] fn push (& 'prev self , item : & 'subs dyn ArgScope < 'subs , 'subs > ,) -> Option < ArgScopeStack < 'prev , 'subs > > ; }
};
}
