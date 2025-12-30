// Generated macro for impl_238 (impl)
macro_rules! Depcrate_astimpl_238 {
() => {
// Module: crate::ast
// Provides: {"impl_238"}
// Dependencies: {}
impl Parse for TemplateParam { fn parse < 'a , 'b > (ctx : & 'a ParseContext , _subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (TemplateParam , IndexStr < 'b >) > { try_begin_parse ! ("TemplateParam" , ctx , input) ; let input = consume (b"T" , input) ? ; let (number , input) = match parse_number (10 , false , input) { Ok ((number , input)) => ((number + 1) as _ , input) , Err (_) => (0 , input) , } ; let input = consume (b"_" , input) ? ; Ok ((TemplateParam (number) , input)) } }
};
}
