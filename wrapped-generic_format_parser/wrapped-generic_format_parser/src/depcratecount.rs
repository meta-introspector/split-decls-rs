// Generated macro for Count (enum)
macro_rules! DepcrateCount {
() => {
// Module: crate
// Provides: {"Count"}
// Dependencies: {}
# [doc = " A count is used for the precision and width parameters of an integer, and"] # [doc = " can reference either an argument or a literal integer."] # [derive (Copy , Clone , Debug , PartialEq)] pub enum Count < 'a > { # [doc = " The count is specified explicitly."] CountIs (usize) , # [doc = " The count is specified by the argument with the given name."] CountIsName (& 'a str , InnerSpan) , # [doc = " The count is specified by the argument at the given index."] CountIsParam (usize) , # [doc = " The count is specified by a star (like in `{:.*}`) that refers to the argument at the given index."] CountIsStar (usize) , # [doc = " The count is implied and cannot be explicitly specified."] CountImplied , }
};
}
