// Generated macro for tests (module)
macro_rules! Depcrate_parser_choicetests {
() => {
// Module: crate::parser::choice
// Provides: {"tests"}
// Dependencies: {}
# [cfg (all (feature = "std" , test))] mod tests { use crate :: parser :: { token :: any , EasyParser } ; use super :: * ; # [test] fn choice_single_parser () { assert ! (choice ((any () ,) ,) . easy_parse ("a") . is_ok ()) ; } }
};
}
