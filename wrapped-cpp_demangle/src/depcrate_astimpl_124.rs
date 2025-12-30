// Generated macro for impl_124 (impl)
macro_rules! Depcrate_astimpl_124 {
() => {
// Module: crate::ast
// Provides: {"impl_124"}
// Dependencies: {}
impl < 'subs , W > Demangle < 'subs , W > for Prefix where W : 'subs + DemangleWrite , { fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; if ctx . is_template_prefix { ctx . push_demangle_node (DemangleNodeType :: TemplatePrefix) ; ctx . is_template_prefix = false ; } else if ctx . is_template_prefix_in_nested_name { ctx . push_demangle_node (DemangleNodeType :: NestedName) ; ctx . is_template_prefix_in_nested_name = false ; } else { ctx . push_demangle_node (DemangleNodeType :: Prefix) ; } let ret = match * self { Prefix :: Unqualified (ref unqualified) => unqualified . demangle (ctx , scope) , Prefix :: Nested (ref prefix , ref unqualified) => { prefix . demangle (ctx , scope) ? ; write ! (ctx , "::") ? ; unqualified . demangle (ctx , scope) } Prefix :: Template (ref prefix , ref args) => { ctx . is_template_prefix = true ; prefix . demangle (ctx , scope) ? ; ctx . is_template_prefix = false ; args . demangle (ctx , scope) } Prefix :: TemplateParam (ref param) => param . demangle (ctx , scope) , Prefix :: Decltype (ref dt) => dt . demangle (ctx , scope) , Prefix :: DataMember (ref prefix , ref member) => { prefix . demangle (ctx , scope) ? ; write ! (ctx , "::") ? ; member . demangle (ctx , scope) } } ; ctx . pop_demangle_node () ; ret } }
};
}
