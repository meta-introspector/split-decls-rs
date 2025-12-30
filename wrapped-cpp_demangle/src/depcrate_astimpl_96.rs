// Generated macro for impl_96 (impl)
macro_rules! Depcrate_astimpl_96 {
() => {
// Module: crate::ast
// Provides: {"impl_96"}
// Dependencies: {}
impl Parse for Name { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (Name , IndexStr < 'b >) > { try_begin_parse ! ("Name" , ctx , input) ; if let Ok ((name , tail)) = try_recurse ! (NestedName :: parse (ctx , subs , input)) { return Ok ((Name :: Nested (name) , tail)) ; } if let Ok ((name , tail)) = try_recurse ! (UnscopedName :: parse (ctx , subs , input)) { if tail . peek () == Some (b'I') { let name = UnscopedTemplateName (name) ; let idx = subs . insert (Substitutable :: UnscopedTemplateName (name)) ; let handle = UnscopedTemplateNameHandle :: BackReference (idx) ; let (args , tail) = TemplateArgs :: parse (ctx , subs , tail) ? ; return Ok ((Name :: UnscopedTemplate (handle , args) , tail)) ; } else { return Ok ((Name :: Unscoped (name) , tail)) ; } } if let Ok ((name , tail)) = try_recurse ! (UnscopedTemplateNameHandle :: parse (ctx , subs , input)) { let (args , tail) = TemplateArgs :: parse (ctx , subs , tail) ? ; return Ok ((Name :: UnscopedTemplate (name , args) , tail)) ; } let (name , tail) = LocalName :: parse (ctx , subs , input) ? ; Ok ((Name :: Local (name) , tail)) } }
};
}
