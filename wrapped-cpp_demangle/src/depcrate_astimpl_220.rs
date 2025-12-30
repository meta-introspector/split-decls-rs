// Generated macro for impl_220 (impl)
macro_rules! Depcrate_astimpl_220 {
() => {
// Module: crate::ast
// Provides: {"impl_220"}
// Dependencies: {}
impl Parse for UnnamedTypeName { fn parse < 'a , 'b > (ctx : & 'a ParseContext , _subs : & 'a mut SubstitutionTable , input : IndexStr < 'b > ,) -> Result < (UnnamedTypeName , IndexStr < 'b >) > { try_begin_parse ! ("UnnamedTypeName" , ctx , input) ; let input = consume (b"Ut" , input) ? ; let (number , input) = match parse_number (10 , false , input) { Ok ((number , input)) => (Some (number as _) , input) , Err (_) => (None , input) , } ; let input = consume (b"_" , input) ? ; Ok ((UnnamedTypeName (number) , input)) } }
};
}
