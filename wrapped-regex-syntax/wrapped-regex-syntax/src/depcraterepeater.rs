// Generated macro for Repeater (enum)
macro_rules! DepcrateRepeater {
() => {
// Module: crate
// Provides: {"Repeater"}
// Dependencies: {}
# [doc = " The type of a repeat operator expression."] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Repeater { # [doc = " Match zero or one (`?`)."] ZeroOrOne , # [doc = " Match zero or more (`*`)."] ZeroOrMore , # [doc = " Match one or more (`+`)."] OneOrMore , # [doc = " Match for at least `min` and at most `max` (`{m,n}`)."] # [doc = ""] # [doc = " When `max` is `None`, there is no upper bound on the number of matches."] Range { # [doc = " Lower bound on the number of matches."] min : u32 , # [doc = " Optional upper bound on the number of matches."] max : Option < u32 > , } , }
};
}
