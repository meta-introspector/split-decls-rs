// Generated macro for chars (function)
macro_rules! Depcratechars {
() => {
// Module: crate
// Provides: {"chars"}
// Dependencies: {}
# [doc = " Computes the diff between the chars of two strings."] pub fn chars < 'a > (left : & 'a str , right : & 'a str) -> Vec < Result < char > > { do_diff (& left . chars () . collect :: < Vec < _ > > () , & right . chars () . collect :: < Vec < _ > > () , | char | * char ,) }
};
}
