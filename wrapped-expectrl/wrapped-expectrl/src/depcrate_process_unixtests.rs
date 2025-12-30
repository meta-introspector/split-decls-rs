// Generated macro for tests (module)
macro_rules! Depcrate_process_unixtests {
() => {
// Module: crate::process::unix
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [cfg (unix)] # [test] fn test_tokenize_command () { let res = tokenize_command ("prog arg1 arg2") ; assert_eq ! (vec ! ["prog" , "arg1" , "arg2"] , res) ; let res = tokenize_command ("prog -k=v") ; assert_eq ! (vec ! ["prog" , "-k=v"] , res) ; let res = tokenize_command ("prog 'my text'") ; assert_eq ! (vec ! ["prog" , "'my text'"] , res) ; let res = tokenize_command (r#"prog "my text""#) ; assert_eq ! (vec ! ["prog" , r#""my text""#] , res) ; } }
};
}
