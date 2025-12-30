// Generated macro for Breakpoints (struct)
macro_rules! DepcrateBreakpoints {
() => {
// Module: crate
// Provides: {"Breakpoints"}
// Dependencies: {}
# [derive (Clone , Debug , PartialEq , Default)] pub struct Breakpoints { # [doc = " An ascending list of breakpoints. All elements must be between 0 and length exclusive."] pub breakpoints : Vec < usize > , # [doc = " The total length; i.e., the limit of the final word."] pub length : usize , }
};
}
