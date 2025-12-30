// Generated macro for Statement (enum)
macro_rules! Depcrate_grammarStatement {
() => {
// Module: crate::grammar
// Provides: {"Statement"}
// Dependencies: {}
# [derive (Debug)] # [derive (Clone)] # [derive (PartialEq)] pub enum Statement { Assign (String , Expr) , Output (Expr) , If (Expr , Comparator , Expr , Block , Option < Block >) , While (Expr , Comparator , Expr , Block) , Loop (Expr , Block) , }
};
}
