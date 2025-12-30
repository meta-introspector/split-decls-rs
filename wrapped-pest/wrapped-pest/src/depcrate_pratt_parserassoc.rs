// Generated macro for Assoc (enum)
macro_rules! Depcrate_pratt_parserAssoc {
() => {
// Module: crate::pratt_parser
// Provides: {"Assoc"}
// Dependencies: {}
# [doc = " Associativity of an infix binary operator, used by [`Op::infix(Assoc)`]."] # [doc = ""] # [doc = " [`Op::infix(Assoc)`]: struct.Op.html"] # [derive (Clone , Copy , Debug , Eq , PartialEq)] pub enum Assoc { # [doc = " Left operator associativity. Evaluate expressions from left-to-right."] Left , # [doc = " Right operator associativity. Evaluate expressions from right-to-left."] Right , }
};
}
