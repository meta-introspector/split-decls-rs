// Generated macro for body (function)
macro_rules! Depcrate_property_test_codegen_test_bodybody {
() => {
// Module: crate::property_test::codegen::test_body
// Provides: {"body"}
// Dependencies: {}
# [doc = " Generate the new test body by putting the struct and arbitrary impl at the start, then adding"] # [doc = " the usual glue that `proptest!` adds"] pub (super) fn body (block : Block , args : & [Argument] , struct_and_impl : TokenStream , fn_name : & Ident , ret_ty : & ReturnType , options : & Options ,) -> Block { let struct_name = struct_name (fn_name) ; let errors = & options . errors ; let struct_fields = args . iter () . enumerate () . map (| (index , arg) | { let pat = arg . pat_ty . pat . as_ref () ; let field_name = nth_field_name (args , index) ; match pat { Pat :: Ident (i) => match i . mutability { Some (mutability) => quote ! (# mutability # field_name ,) , None => quote ! (# field_name ,) , } , _ => quote ! (# field_name : # pat ,) , } }) ; let struct_pattern = quote ! { # struct_name { # (# struct_fields) * } } ; let handle_result = handle_result (ret_ty) ; let config = make_config (options . config . as_ref ()) ; let tokens = quote ! ({ # (# errors) * # struct_and_impl # config let mut runner = :: proptest :: test_runner :: TestRunner :: new (config) ; let result = runner . run (&:: proptest :: strategy :: Strategy :: prop_map (:: proptest :: prelude :: any ::<# struct_name > () , | values | { :: proptest :: sugar :: NamedArguments (stringify ! (# struct_name) , values) }) , |:: proptest :: sugar :: NamedArguments (_ , # struct_pattern) | { let result = # block ; # handle_result } ,) ; match result { Ok (()) => { } Err (e) => panic ! ("{}" , e) , } }) ; parse2 (tokens) . unwrap () }
};
}
