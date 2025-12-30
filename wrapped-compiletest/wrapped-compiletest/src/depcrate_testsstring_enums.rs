// Generated macro for string_enums (function)
macro_rules! Depcrate_testsstring_enums {
() => {
// Module: crate::tests
// Provides: {"string_enums"}
// Dependencies: {}
# [test] fn string_enums () { crate :: util :: string_enum ! { # [derive (Clone , Copy , Debug , PartialEq)] enum Animal { Cat => "meow" , Dog => "woof" , } } assert_eq ! (Animal :: VARIANTS . len () , 2) ; assert_eq ! (Animal :: STR_VARIANTS . len () , 2) ; assert_eq ! (Animal :: Cat , "meow" . parse () . unwrap ()) ; assert_eq ! (Animal :: Dog , "woof" . parse () . unwrap ()) ; let animal = "nya" . parse :: < Animal > () ; assert_eq ! ("unknown `Animal` variant: `nya`" , animal . unwrap_err ()) ; }
};
}
