// Generated macro for tests (module)
macro_rules! Depcrate_ast_datatests {
() => {
// Module: crate::ast::data
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; fn token_stream_to_fields (input : TokenStream) -> Fields < syn :: Field > { Fields :: try_from (& { if let syn :: Data :: Struct (s) = syn :: parse2 :: < syn :: DeriveInput > (input) . unwrap () . data { s . fields } else { panic ! () ; } }) . unwrap () } # [test] fn test_style_eq () { struct _AssertEq where Style : Eq ; } # [test] fn test_fields_to_tokens_struct () { let reference = quote ! ({ executable : String , args : Vec < String >, env : Vec < String >, index : usize , optional : Option < String >, current_dir : String , }) ; let input = quote ! (struct ExampleTest # reference) ; let fields = token_stream_to_fields (input) ; let mut result = quote ! () ; fields . to_tokens (& mut result) ; assert_eq ! (result . to_string () , reference . to_string ()) ; } # [test] fn test_fields_to_tokens_tuple () { let reference = quote ! ((u64 , usize , &'a T)) ; let input = quote ! (struct ExampleTest # reference ;) ; let fields = token_stream_to_fields (input) ; let mut result = quote ! () ; fields . to_tokens (& mut result) ; assert_eq ! (result . to_string () , reference . to_string ()) ; } }
};
}
