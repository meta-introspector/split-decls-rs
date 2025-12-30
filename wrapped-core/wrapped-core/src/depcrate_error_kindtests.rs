// Generated macro for tests (module)
macro_rules! Depcrate_error_kindtests {
() => {
// Module: crate::error::kind
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: { ErrorUnknownValue , UnknownValuePosition } ; # [doc = " Make sure that an unknown field error with no alts or suggestions has"] # [doc = " only the relevant information and no fragments of other sentences."] # [test] fn present_no_alts () { let err = ErrorUnknownValue :: new (UnknownValuePosition :: Field , "hello") ; assert_eq ! (& err . to_string () , "Unknown field: `hello`") ; } # [test] fn present_few_alts () { let err = ErrorUnknownValue :: with_alts (UnknownValuePosition :: Field , "hello" , & ["world" , "friend"] ,) ; assert ! (err . to_string () . contains ("`friend`")) ; } }
};
}
