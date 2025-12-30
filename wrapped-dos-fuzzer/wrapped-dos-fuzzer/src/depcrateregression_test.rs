// Generated macro for regression_test (function)
macro_rules! Depcrateregression_test {
() => {
// Module: crate
// Provides: {"regression_test"}
// Dependencies: {}
# [doc = " Tests patterns of previously known super-linear parsing behaviour."] # [doc = ""] # [doc = " Returns the exit code. 0 if all tests passed and 1 otherwise."] fn regression_test () -> i32 { let mut exit_code = 0 ; let mut check_pattern = | pat | match test (& pat) { PatternResult :: Linear (_) => () , _ => exit_code = 1 , } ; check_pattern ("[](" . into ()) ; check_pattern ("``\\" . into ()) ; check_pattern ("a***" . into ()) ; check_pattern (Pattern { prefix : "" . into () , repeating_pattern : "* " . into () , suffix : "a" . into () , }) ; check_pattern ("[ (](" . into ()) ; check_pattern ("[*_a" . into ()) ; check_pattern ("a <![CDATA[" . into ()) ; check_pattern ("a <!A" . into ()) ; check_pattern ("a<?" . into ()) ; check_pattern ("[[]()" . into ()) ; check_pattern ("[](<" . into ()) ; check_pattern ("[\"[]]\\(" . into ()) ; check_pattern (")-\r%<[" . into ()) ; check_pattern ("\u{0}[@[{<" . into ()) ; check_pattern ("a <!A " . into ()) ; check_pattern ("a <? " . into ()) ; check_pattern ("[ (]( " . into ()) ; check_pattern (Pattern { prefix : "" . into () , repeating_pattern : "`a`" . into () , suffix : "`" . into () , }) ; check_pattern ("\\``" . into ()) ; check_pattern ("a***b~~" . into ()) ; check_pattern ("*~~\u{a0}" . into ()) ; check_pattern ("[*_a" . into ()) ; check_pattern ("a***_b__" . into ()) ; check_pattern ("a***" . into ()) ; check_pattern ("[[]()" . into ()) ; check_pattern ("[a](<" . into ()) ; check_pattern ("!-- <" . into ()) ; exit_code }
};
}
