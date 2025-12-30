// Generated macro for tests (module)
macro_rules! Depcrate_parse_servicetests {
() => {
// Module: crate::parse::service
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use std :: fs ; use super :: * ; # [test] fn test_parser () { for entry in fs :: read_dir ("tests/services") . unwrap () { let entry = entry . unwrap () ; eprintln ! ("Parsing file {}" , entry . path () . display ()) ; GraphQLParser :: parse (Rule :: service_document , & fs :: read_to_string (entry . path ()) . unwrap () ,) . unwrap () ; } } # [test] fn test_parser_ast () { for entry in fs :: read_dir ("tests/services") . unwrap () { let entry = entry . unwrap () ; parse_schema (fs :: read_to_string (entry . path ()) . unwrap ()) . unwrap () ; } } }
};
}
