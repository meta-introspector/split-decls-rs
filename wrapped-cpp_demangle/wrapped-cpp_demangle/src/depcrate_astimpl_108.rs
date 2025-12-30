// Generated macro for impl_108 (impl)
macro_rules! Depcrate_astimpl_108 {
() => {
// Module: crate::ast
// Provides: {"impl_108"}
// Dependencies: {}
impl Parse for UnscopedTemplateNameHandle { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (UnscopedTemplateNameHandle , IndexStr < 'b >) > { try_begin_parse ! ("UnscopedTemplateNameHandle" , ctx , input) ; if let Ok ((name , tail)) = try_recurse ! (UnscopedName :: parse (ctx , subs , input)) { let name = UnscopedTemplateName (name) ; let idx = subs . insert (Substitutable :: UnscopedTemplateName (name)) ; let handle = UnscopedTemplateNameHandle :: BackReference (idx) ; return Ok ((handle , tail)) ; } let (sub , tail) = Substitution :: parse (ctx , subs , input) ? ; match sub { Substitution :: WellKnown (component) => { Ok ((UnscopedTemplateNameHandle :: WellKnown (component) , tail)) } Substitution :: BackReference (idx) => { Ok ((UnscopedTemplateNameHandle :: BackReference (idx) , tail)) } } } }
};
}
