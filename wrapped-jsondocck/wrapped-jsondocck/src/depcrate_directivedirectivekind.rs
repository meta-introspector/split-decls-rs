// Generated macro for DirectiveKind (enum)
macro_rules! Depcrate_directiveDirectiveKind {
() => {
// Module: crate::directive
// Provides: {"DirectiveKind"}
// Dependencies: {}
# [derive (Debug)] pub enum DirectiveKind { # [doc = " `//@ has <path>`"] # [doc = ""] # [doc = " Checks the path exists."] HasPath , # [doc = " `//@ has <path> <value>`"] # [doc = ""] # [doc = " Check one thing at the path  is equal to the value."] HasValue { value : String } , # [doc = " `//@ !has <path>`"] # [doc = ""] # [doc = " Checks the path doesn't exist."] HasNotPath , # [doc = " `//@ !has <path> <value>`"] # [doc = ""] # [doc = " Checks the path exists, but doesn't have the given value."] HasNotValue { value : String } , # [doc = " `//@ is <path> <value>`"] # [doc = ""] # [doc = " Check the path is the given value."] Is { value : String } , # [doc = " `//@ is <path> <value> <value>...`"] # [doc = ""] # [doc = " Check that the path matches to exactly every given value."] IsMany { values : Vec < String > } , # [doc = " `//@ !is <path> <value>`"] # [doc = ""] # [doc = " Check the path isn't the given value."] IsNot { value : String } , # [doc = " `//@ count <path> <value>`"] # [doc = ""] # [doc = " Check the path has the expected number of matches."] CountIs { expected : usize } , # [doc = " `//@ set <name> = <path>`"] Set { variable : String } , }
};
}
