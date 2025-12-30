// Generated macro for tests (module)
macro_rules! Depcrate_parser_sequencetests {
() => {
// Module: crate::parser::sequence
// Provides: {"tests"}
// Dependencies: {}
# [cfg (all (feature = "std" , test))] mod tests { use crate :: parser :: { token :: any , EasyParser } ; # [test] fn sequence_single_parser () { assert ! ((any () ,) . easy_parse ("a") . is_ok ()) ; } }
};
}
