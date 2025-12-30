// Generated macro for to_camel_case (function)
macro_rules! Depcrate_str_utilsto_camel_case {
() => {
// Module: crate::str_utils
// Provides: {"to_camel_case"}
// Dependencies: {}
# [doc = " Returns a `CamelCase` version of the input"] # [doc = " ```no_run"] # [doc = " use clippy_utils::str_utils::to_camel_case;"] # [doc = " assert_eq!(to_camel_case(\"abc_def\"), \"AbcDef\");"] # [doc = " assert_eq!(to_camel_case(\"a_b_c_d\"), \"ABCD\");"] # [doc = " assert_eq!(to_camel_case(\"abc_d_d\"), \"AbcDD\");"] # [doc = " assert_eq!(to_camel_case(\"abc1_d_d\"), \"Abc1DD\");"] # [doc = " ```"] pub fn to_camel_case (item_name : & str) -> String { let mut s = String :: new () ; let mut up = true ; for c in item_name . chars () { if c . is_uppercase () { return item_name . to_string () ; } if c == '_' { up = true ; continue ; } if up { up = false ; s . extend (c . to_uppercase ()) ; } else { s . push (c) ; } } s }
};
}
