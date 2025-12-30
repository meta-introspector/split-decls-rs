// Generated macro for concatenate (function)
macro_rules! Depcrate_optimizer_concatenatorconcatenate {
() => {
// Module: crate::optimizer::concatenator
// Provides: {"concatenate"}
// Dependencies: {}
pub fn concatenate (rule : Rule) -> Rule { let Rule { name , ty , expr } = rule ; Rule { name , ty , expr : expr . map_bottom_up (| expr | { if ty == RuleType :: Atomic { match expr { Expr :: Seq (lhs , rhs) => match (* lhs , * rhs) { (Expr :: Str (lhs) , Expr :: Str (rhs)) => Expr :: Str (lhs + & rhs) , (Expr :: Insens (lhs) , Expr :: Insens (rhs)) => Expr :: Insens (lhs + & rhs) , (lhs , rhs) => Expr :: Seq (Box :: new (lhs) , Box :: new (rhs)) , } , expr => expr , } } else { expr } }) , } }
};
}
