// Generated macro for to_snake_case (function)
macro_rules! Depcrate_str_utilsto_snake_case {
() => {
// Module: crate::str_utils
// Provides: {"to_snake_case"}
// Dependencies: {}
# [doc = " Returns a `snake_case` version of the input"] # [doc = " ```no_run"] # [doc = " use clippy_utils::str_utils::to_snake_case;"] # [doc = " assert_eq!(to_snake_case(\"AbcDef\"), \"abc_def\");"] # [doc = " assert_eq!(to_snake_case(\"ABCD\"), \"a_b_c_d\");"] # [doc = " assert_eq!(to_snake_case(\"AbcDD\"), \"abc_d_d\");"] # [doc = " assert_eq!(to_snake_case(\"Abc1DD\"), \"abc1_d_d\");"] # [doc = " ```"] pub fn to_snake_case (name : & str) -> String { let mut s = String :: new () ; for (i , c) in name . chars () . enumerate () { if c . is_uppercase () { if i != 0 { s . push ('_') ; } s . extend (c . to_lowercase ()) ; } else { s . push (c) ; } } s }
};
}
