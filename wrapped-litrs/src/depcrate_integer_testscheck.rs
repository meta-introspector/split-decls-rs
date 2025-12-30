// Generated macro for check (function)
macro_rules! Depcrate_integer_testscheck {
() => {
// Module: crate::integer::tests
// Provides: {"check"}
// Dependencies: {}
# [track_caller] fn check < T : FromIntegerLiteral + PartialEq + Debug + Display > (input : & str , value : T , base : IntegerBase , main_part : & str , type_suffix : Option < Ty > ,) { let expected_integer = IntegerLit { raw : input , start_main_part : base . prefix () . len () , end_main_part : base . prefix () . len () + main_part . len () , base , } ; assert_parse_ok_eq (input , IntegerLit :: parse (input) , expected_integer . clone () , "IntegerLit::parse") ; assert_parse_ok_eq (input , Literal :: parse (input) , Literal :: Integer (expected_integer) , "Literal::parse") ; assert_roundtrip (expected_integer . to_owned () , input) ; assert_eq ! (Ty :: from_suffix (IntegerLit :: parse (input) . unwrap () . suffix ()) , type_suffix) ; let actual_value = IntegerLit :: parse (input) . unwrap () . value :: < T > () . unwrap_or_else (| | panic ! ("unexpected overflow in `IntegerLit::value` for `{}`" , input)) ; if actual_value != value { panic ! ("Parsing int literal `{input}` should give value `{value}`, \
            but actually resulted in `{actual_value}`") ; } }
};
}
