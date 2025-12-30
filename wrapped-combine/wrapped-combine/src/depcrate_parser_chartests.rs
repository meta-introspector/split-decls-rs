// Generated macro for tests (module)
macro_rules! Depcrate_parser_chartests {
() => {
// Module: crate::parser::char
// Provides: {"tests"}
// Dependencies: {}
# [cfg (all (feature = "std" , test))] mod tests { use crate :: { parser :: EasyParser , stream :: { easy :: { Error , Errors } , position :: { self , SourcePosition } , } , } ; use super :: * ; # [test] fn space_error () { let result = space () . easy_parse ("") ; assert ! (result . is_err ()) ; assert_eq ! (result . unwrap_err () . errors , vec ! [Error :: end_of_input () , Error :: Expected ("whitespace" . into ())]) ; } # [test] fn string_committed () { let result = string ("a") . easy_parse (position :: Stream :: new ("b")) ; assert ! (result . is_err ()) ; assert_eq ! (result . unwrap_err () . position , SourcePosition { line : 1 , column : 1 }) ; } # [test] fn string_error () { let result = string ("abc") . easy_parse (position :: Stream :: new ("bc")) ; assert_eq ! (result , Err (Errors { position : SourcePosition { line : 1 , column : 1 } , errors : vec ! [Error :: Unexpected ('b' . into ()) , Error :: Expected ("abc" . into ())] , })) ; } }
};
}
