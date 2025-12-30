// Generated macro for tests (module)
macro_rules! Depcrate_parser_rangetests {
() => {
// Module: crate::parser::range
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: Parser ; use super :: * ; # [test] fn take_while_test () { let result = take_while (| c : char | c . is_digit (10)) . parse ("123abc") ; assert_eq ! (result , Ok (("123" , "abc"))) ; let result = take_while (| c : char | c . is_digit (10)) . parse ("abc") ; assert_eq ! (result , Ok (("" , "abc"))) ; } # [test] fn take_while1_test () { let result = take_while1 (| c : char | c . is_digit (10)) . parse ("123abc") ; assert_eq ! (result , Ok (("123" , "abc"))) ; let result = take_while1 (| c : char | c . is_digit (10)) . parse ("abc") ; assert ! (result . is_err ()) ; } # [test] fn range_string_no_char_boundary_error () { let mut parser = range ("hello") ; let result = parser . parse ("hell\u{00EE} world") ; assert ! (result . is_err ()) ; } # [test] fn take_until_range_1 () { let result = take_until_range ("\"") . parse ("Foo baz bar quux\"") ; assert_eq ! (result , Ok (("Foo baz bar quux" , "\""))) ; } # [test] fn take_until_range_2 () { let result = take_until_range ("===") . parse ("if ((pointless_comparison == 3) === true) {") ; assert_eq ! (result , Ok (("if ((pointless_comparison == 3) " , "=== true) {"))) ; } # [test] fn take_until_range_unicode_1 () { let result = take_until_range ("🦀") . parse ("😃 Ferris the friendly rustacean 🦀 and his snake friend 🐍") ; assert_eq ! (result , Ok (("😃 Ferris the friendly rustacean " , "🦀 and his snake friend 🐍"))) ; } # [test] fn take_until_range_unicode_2 () { let result = take_until_range ("⁘⁙/⁘") . parse ("⚙️🛠️🦀=🏎️⁘⁙⁘⁘⁙/⁘⁘⁙/⁘") ; assert_eq ! (result , Ok (("⚙️🛠️🦀=🏎️⁘⁙⁘" , "⁘⁙/⁘⁘⁙/⁘"))) ; } }
};
}
