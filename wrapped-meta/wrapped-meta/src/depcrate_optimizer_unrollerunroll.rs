// Generated macro for unroll (function)
macro_rules! Depcrate_optimizer_unrollerunroll {
() => {
// Module: crate::optimizer::unroller
// Provides: {"unroll"}
// Dependencies: {}
pub fn unroll (rule : Rule) -> Rule { let Rule { name , ty , expr } = rule ; Rule { name , ty , expr : expr . map_bottom_up (| expr | match expr { # [cfg (not (feature = "grammar-extras"))] Expr :: RepOnce (expr) => Expr :: Seq (expr . clone () , Box :: new (Expr :: Rep (expr))) , Expr :: RepExact (expr , num) => (1 .. num + 1) . map (| _ | * expr . clone ()) . rev () . fold (None , | rep , expr | match rep { None => Some (expr) , Some (rep) => Some (Expr :: Seq (Box :: new (expr) , Box :: new (rep))) , }) . unwrap () , Expr :: RepMin (expr , min) => (1 .. min + 2) . map (| i | { if i <= min { * expr . clone () } else { Expr :: Rep (expr . clone ()) } }) . rev () . fold (None , | rep , expr | match rep { None => Some (expr) , Some (rep) => Some (Expr :: Seq (Box :: new (expr) , Box :: new (rep))) , }) . unwrap () , Expr :: RepMax (expr , max) => (1 .. max + 1) . map (| _ | Expr :: Opt (expr . clone ())) . rev () . fold (None , | rep , expr | match rep { None => Some (expr) , Some (rep) => Some (Expr :: Seq (Box :: new (expr) , Box :: new (rep))) , }) . unwrap () , Expr :: RepMinMax (expr , min , max) => (1 .. max + 1) . map (| i | { if i <= min { * expr . clone () } else { Expr :: Opt (expr . clone ()) } }) . rev () . fold (None , | rep , expr | match rep { None => Some (expr) , Some (rep) => Some (Expr :: Seq (Box :: new (expr) , Box :: new (rep))) , }) . unwrap () , expr => expr , }) , } }
};
}
