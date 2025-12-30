// Generated macro for impl_48 (impl)
macro_rules! Depcrate_astimpl_48 {
() => {
// Module: crate::ast
// Provides: {"impl_48"}
// Dependencies: {}
impl < 'prev , 'subs > ArgScopeStackExt < 'prev , 'subs > for Option < ArgScopeStack < 'prev , 'subs > > { fn push (& 'prev self , item : & 'subs dyn ArgScope < 'subs , 'subs > ,) -> Option < ArgScopeStack < 'prev , 'subs > > { log ! ("ArgScopeStack::push: {:?}" , item) ; Some (ArgScopeStack { prev : self . as_ref () , in_arg : None , item : item , }) } }
};
}
