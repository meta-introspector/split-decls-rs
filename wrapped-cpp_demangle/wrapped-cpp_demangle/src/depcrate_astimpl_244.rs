// Generated macro for impl_244 (impl)
macro_rules! Depcrate_astimpl_244 {
() => {
// Module: crate::ast
// Provides: {"impl_244"}
// Dependencies: {}
impl Parse for TemplateTemplateParamHandle { fn parse < 'a , 'b > (ctx : & 'a ParseContext , subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (TemplateTemplateParamHandle , IndexStr < 'b >) > { try_begin_parse ! ("TemplateTemplateParamHandle" , ctx , input) ; if let Ok ((sub , tail)) = try_recurse ! (Substitution :: parse (ctx , subs , input)) { match sub { Substitution :: WellKnown (component) => { return Ok ((TemplateTemplateParamHandle :: WellKnown (component) , tail)) ; } Substitution :: BackReference (idx) => { return Ok ((TemplateTemplateParamHandle :: BackReference (idx) , tail)) ; } } } let (param , tail) = TemplateParam :: parse (ctx , subs , input) ? ; let ttp = TemplateTemplateParam (param) ; let ttp = Substitutable :: TemplateTemplateParam (ttp) ; let idx = subs . insert (ttp) ; let handle = TemplateTemplateParamHandle :: BackReference (idx) ; Ok ((handle , tail)) } }
};
}
