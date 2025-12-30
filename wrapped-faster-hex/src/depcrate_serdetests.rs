// Generated macro for tests (module)
macro_rules! Depcrate_serdetests {
() => {
// Module: crate::serde
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: { nopfx_ignorecase , nopfx_lowercase , nopfx_uppercase , option_nopfx_ignorecase , option_nopfx_lowercase , option_nopfx_uppercase , option_withpfx_ignorecase , option_withpfx_lowercase , option_withpfx_uppercase , withpfx_ignorecase , withpfx_lowercase , withpfx_uppercase , } ; use crate as faster_hex ; use bytes :: Bytes ; use proptest :: proptest ; use serde :: { Deserialize , Serialize } ; # [derive (Debug , PartialEq , Eq , Serialize , Deserialize)] struct Simple { # [serde (with = "faster_hex")] bar : Vec < u8 > , } # [test] fn test_deserialize_escaped () { let x : Simple = serde_json :: from_str (r#"{
            "bar": "\u0030x\u00303"
        }"# ,) . unwrap () ; assert_eq ! (x . bar , b"\x03") ; } fn _test_simple (src : & str) { let simple = Simple { bar : src . into () } ; let result = serde_json :: to_string (& simple) ; assert ! (result . is_ok ()) ; let result = result . unwrap () ; assert ! (result . starts_with (r#"{"bar":"0x"#)) ; assert ! (result [7 ..] . chars () . all (| c | ! c . is_uppercase ())) ; let decode_simple = serde_json :: from_str :: < Simple > (& result) ; assert ! (decode_simple . is_ok ()) ; assert_eq ! (decode_simple . unwrap () , simple) ; } proptest ! { # [test] fn test_simple (ref s in ".*") { _test_simple (s) ; } } # [derive (Debug , PartialEq , Eq , Serialize , Deserialize)] struct Foo { # [serde (with = "nopfx_lowercase")] bar_nopfx_lowercase_vec : Vec < u8 > , # [serde (with = "nopfx_lowercase")] bar_nopfx_lowercase_bytes : Bytes , # [serde (with = "withpfx_lowercase")] bar_withpfx_lowercase_vec : Vec < u8 > , # [serde (with = "withpfx_lowercase")] bar_withpfx_lowercase_bytes : Bytes , # [serde (with = "nopfx_uppercase")] bar_nopfx_uppercase_vec : Vec < u8 > , # [serde (with = "nopfx_uppercase")] bar_nopfx_uppercase_bytes : Bytes , # [serde (with = "withpfx_uppercase")] bar_withpfx_uppercase_vec : Vec < u8 > , # [serde (with = "withpfx_uppercase")] bar_withpfx_uppercase_bytes : Bytes , # [serde (with = "withpfx_ignorecase")] bar_withpfx_ignorecase_vec : Vec < u8 > , # [serde (with = "withpfx_ignorecase")] bar_withpfx_ignorecase_bytes : Bytes , # [serde (with = "nopfx_ignorecase")] bar_nopfx_ignorecase_vec : Vec < u8 > , # [serde (with = "nopfx_ignorecase")] bar_nopfx_ignorecase_bytes : Bytes , # [serde (with = "option_nopfx_ignorecase")] bar_nopfx_ignorecase_vec_option : Option < Vec < u8 > > , # [serde (with = "option_nopfx_ignorecase")] bar_nopfx_ignorecase_bytes_option : Option < Bytes > , # [serde (with = "option_withpfx_ignorecase")] bar_withpfx_ignorecase_vec_option : Option < Vec < u8 > > , # [serde (with = "option_withpfx_ignorecase")] bar_withpfx_ignorecase_bytes_option : Option < Bytes > , # [serde (with = "option_nopfx_lowercase")] bar_nopfx_lowercase_vec_option : Option < Vec < u8 > > , # [serde (with = "option_nopfx_lowercase")] bar_nopfx_lowercase_bytes_option : Option < Bytes > , # [serde (with = "option_withpfx_lowercase")] bar_withpfx_lowercase_vec_option : Option < Vec < u8 > > , # [serde (with = "option_withpfx_lowercase")] bar_withpfx_lowercase_bytes_option : Option < Bytes > , # [serde (with = "option_nopfx_uppercase")] bar_nopfx_uppercase_vec_option : Option < Vec < u8 > > , # [serde (with = "option_nopfx_uppercase")] bar_nopfx_uppercase_bytes_option : Option < Bytes > , # [serde (with = "option_withpfx_uppercase")] bar_withpfx_uppercase_vec_option : Option < Vec < u8 > > , # [serde (with = "option_withpfx_uppercase")] bar_withpfx_uppercase_bytes_option : Option < Bytes > , } # [test] fn test_serde_default () { { let foo_defuault = Foo { bar_nopfx_lowercase_vec : vec ! [] , bar_nopfx_lowercase_bytes : Default :: default () , bar_withpfx_lowercase_vec : vec ! [] , bar_withpfx_lowercase_bytes : Default :: default () , bar_nopfx_uppercase_vec : vec ! [] , bar_nopfx_uppercase_bytes : Default :: default () , bar_withpfx_uppercase_vec : vec ! [] , bar_withpfx_uppercase_bytes : Default :: default () , bar_withpfx_ignorecase_vec : vec ! [] , bar_withpfx_ignorecase_bytes : Default :: default () , bar_nopfx_ignorecase_vec : vec ! [] , bar_nopfx_ignorecase_bytes : Default :: default () , bar_nopfx_ignorecase_vec_option : Default :: default () , bar_nopfx_ignorecase_bytes_option : Default :: default () , bar_withpfx_ignorecase_vec_option : Default :: default () , bar_withpfx_ignorecase_bytes_option : Default :: default () , bar_nopfx_lowercase_vec_option : Default :: default () , bar_nopfx_lowercase_bytes_option : Default :: default () , bar_withpfx_lowercase_vec_option : Default :: default () , bar_withpfx_lowercase_bytes_option : Default :: default () , bar_nopfx_uppercase_vec_option : Default :: default () , bar_nopfx_uppercase_bytes_option : Default :: default () , bar_withpfx_uppercase_vec_option : Default :: default () , bar_withpfx_uppercase_bytes_option : Default :: default () , } ; let serde_result = serde_json :: to_string (& foo_defuault) . unwrap () ; let expect = r#"
{"bar_nopfx_lowercase_vec":"",
"bar_nopfx_lowercase_bytes":"",
"bar_withpfx_lowercase_vec":"0x",
"bar_withpfx_lowercase_bytes":"0x",
"bar_nopfx_uppercase_vec":"",
"bar_nopfx_uppercase_bytes":"",
"bar_withpfx_uppercase_vec":"0x",
"bar_withpfx_uppercase_bytes":"0x",
"bar_withpfx_ignorecase_vec":"0x",
"bar_withpfx_ignorecase_bytes":"0x",
"bar_nopfx_ignorecase_vec":"",
"bar_nopfx_ignorecase_bytes":"",
"bar_nopfx_ignorecase_vec_option":null,
"bar_nopfx_ignorecase_bytes_option":null,
"bar_withpfx_ignorecase_vec_option":null,
"bar_withpfx_ignorecase_bytes_option":null,
"bar_nopfx_lowercase_vec_option":null,
"bar_nopfx_lowercase_bytes_option":null,
"bar_withpfx_lowercase_vec_option":null,
"bar_withpfx_lowercase_bytes_option":null,
"bar_nopfx_uppercase_vec_option":null,
"bar_nopfx_uppercase_bytes_option":null,
"bar_withpfx_uppercase_vec_option":null,
"bar_withpfx_uppercase_bytes_option":null}"# ; let expect = expect . replace ('\n' , "") ; assert_eq ! (serde_result , expect) ; let foo_src : Foo = serde_json :: from_str (& serde_result) . unwrap () ; assert_eq ! (foo_defuault , foo_src) ; } } fn _test_serde (src : & str) { let foo = Foo { bar_nopfx_lowercase_vec : Vec :: from (src) , bar_nopfx_lowercase_bytes : Bytes :: from (Vec :: from (src)) , bar_withpfx_lowercase_vec : Vec :: from (src) , bar_withpfx_lowercase_bytes : Bytes :: from (Vec :: from (src)) , bar_nopfx_uppercase_vec : Vec :: from (src) , bar_nopfx_uppercase_bytes : Bytes :: from (Vec :: from (src)) , bar_withpfx_uppercase_vec : Vec :: from (src) , bar_withpfx_uppercase_bytes : Bytes :: from (Vec :: from (src)) , bar_withpfx_ignorecase_vec : Vec :: from (src) , bar_withpfx_ignorecase_bytes : Bytes :: from (Vec :: from (src)) , bar_nopfx_ignorecase_vec : Vec :: from (src) , bar_nopfx_ignorecase_bytes : Bytes :: from (Vec :: from (src)) , bar_withpfx_ignorecase_vec_option : Some (Vec :: from (src)) , bar_nopfx_ignorecase_bytes_option : Some (Bytes :: from (Vec :: from (src))) , bar_nopfx_ignorecase_vec_option : Some (Vec :: from (src)) , bar_withpfx_ignorecase_bytes_option : Some (Bytes :: from (Vec :: from (src))) , bar_nopfx_lowercase_vec_option : Some (Vec :: from (src)) , bar_nopfx_lowercase_bytes_option : Some (Bytes :: from (Vec :: from (src))) , bar_withpfx_lowercase_vec_option : Some (Vec :: from (src)) , bar_withpfx_lowercase_bytes_option : Some (Bytes :: from (Vec :: from (src))) , bar_nopfx_uppercase_vec_option : Some (Vec :: from (src)) , bar_nopfx_uppercase_bytes_option : Some (Bytes :: from (Vec :: from (src))) , bar_withpfx_uppercase_vec_option : Some (Vec :: from (src)) , bar_withpfx_uppercase_bytes_option : Some (Bytes :: from (Vec :: from (src))) , } ; let hex_str = hex :: encode (src) ; let hex_str_upper = hex :: encode_upper (src) ; let serde_result = serde_json :: to_string (& foo) . unwrap () ; let expect = format ! (r#"{{"bar_nopfx_lowercase_vec":"{}",
"bar_nopfx_lowercase_bytes":"{}",
"bar_withpfx_lowercase_vec":"0x{}",
"bar_withpfx_lowercase_bytes":"0x{}",
"bar_nopfx_uppercase_vec":"{}",
"bar_nopfx_uppercase_bytes":"{}",
"bar_withpfx_uppercase_vec":"0x{}",
"bar_withpfx_uppercase_bytes":"0x{}",
"bar_withpfx_ignorecase_vec":"0x{}",
"bar_withpfx_ignorecase_bytes":"0x{}",
"bar_nopfx_ignorecase_vec":"{}",
"bar_nopfx_ignorecase_bytes":"{}",
"bar_nopfx_ignorecase_vec_option":"{}",
"bar_nopfx_ignorecase_bytes_option":"{}",
"bar_withpfx_ignorecase_vec_option":"0x{}",
"bar_withpfx_ignorecase_bytes_option":"0x{}",
"bar_nopfx_lowercase_vec_option":"{}",
"bar_nopfx_lowercase_bytes_option":"{}",
"bar_withpfx_lowercase_vec_option":"0x{}",
"bar_withpfx_lowercase_bytes_option":"0x{}",
"bar_nopfx_uppercase_vec_option":"{}",
"bar_nopfx_uppercase_bytes_option":"{}",
"bar_withpfx_uppercase_vec_option":"0x{}",
"bar_withpfx_uppercase_bytes_option":"0x{}"}}"# , hex_str , hex_str , hex_str , hex_str , hex_str_upper , hex_str_upper , hex_str_upper , hex_str_upper , hex_str , hex_str , hex_str , hex_str , hex_str , hex_str , hex_str , hex_str , hex_str , hex_str , hex_str , hex_str , hex_str_upper , hex_str_upper , hex_str_upper , hex_str_upper ,) ; let expect = expect . replace ('\n' , "") ; assert_eq ! (serde_result , expect) ; let foo_src : Foo = serde_json :: from_str (& serde_result) . unwrap () ; assert_eq ! (foo , foo_src) ; } proptest ! { # [test] fn test_serde (ref s in ".*") { _test_serde (s) ; } } fn _test_serde_deserialize (src : & str) { # [derive (Debug , PartialEq , Eq , Serialize , Deserialize)] struct FooNoPfxLower { # [serde (with = "nopfx_lowercase")] bar : Vec < u8 > , } # [derive (Debug , PartialEq , Eq , Serialize , Deserialize)] struct FooWithPfxLower { # [serde (with = "withpfx_lowercase")] bar : Vec < u8 > , } # [derive (Debug , PartialEq , Eq , Serialize , Deserialize)] struct FooNoPfxUpper { # [serde (with = "nopfx_uppercase")] bar : Vec < u8 > , } # [derive (Debug , PartialEq , Eq , Serialize , Deserialize)] struct FooWithPfxUpper { # [serde (with = "withpfx_uppercase")] bar : Vec < u8 > , } # [derive (Debug , PartialEq , Eq , Serialize , Deserialize)] struct FooNoPfxIgnoreCase { # [serde (with = "nopfx_ignorecase")] bar : Vec < u8 > , } # [derive (Debug , PartialEq , Eq , Serialize , Deserialize)] struct FooWithPfxIgnoreCase { # [serde (with = "withpfx_ignorecase")] bar : Vec < u8 > , } { let hex_foo = serde_json :: to_string (& FooNoPfxLower { bar : src . into () }) . unwrap () ; let foo_pfx : serde_json :: Result < FooWithPfxLower > = serde_json :: from_str (& hex_foo) ; assert ! (foo_pfx . is_err ()) ; assert ! (foo_pfx . unwrap_err () . to_string () . contains ("invalid prefix")) ; } { let foo_lower = serde_json :: to_string (& FooNoPfxLower { bar : src . into () }) . unwrap () ; let foo_upper_result : serde_json :: Result < FooNoPfxUpper > = serde_json :: from_str (& foo_lower) ; if hex :: encode (src) . contains (char :: is_lowercase) { assert ! (foo_upper_result . is_err ()) ; assert ! (foo_upper_result . unwrap_err () . to_string () . contains ("Invalid character")) ; } } } proptest ! { # [test] fn test_serde_deserialize (ref s in ".*") { _test_serde_deserialize (s) ; } } }
};
}
