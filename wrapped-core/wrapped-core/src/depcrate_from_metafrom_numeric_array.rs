// Generated macro for from_numeric_array (macro)
macro_rules! Depcrate_from_metafrom_numeric_array {
() => {
// Module: crate::from_meta
// Provides: {"from_numeric_array"}
// Dependencies: {}
macro_rules ! from_numeric_array { ($ ty : ident) => { # [doc = " Parsing an unsigned integer array, i.e. `example = \"[1, 2, 3, 4]\"`."] impl FromMeta for Vec <$ ty > { fn from_expr (expr : & syn :: Expr) -> Result < Self > { match expr { syn :: Expr :: Array (expr_array) => expr_array . elems . iter () . map (| expr | { let unexpected = || { Error :: custom ("Expected array of unsigned integers") . with_span (expr) } ; match expr { Expr :: Lit (lit) => $ ty :: from_value (& lit . lit) , Expr :: Group (group) => match &* group . expr { Expr :: Lit (lit) => $ ty :: from_value (& lit . lit) , _ => Err (unexpected ()) , } , _ => Err (unexpected ()) , } }) . collect ::< Result < Vec <$ ty >>> () , syn :: Expr :: Lit (expr_lit) => Self :: from_value (& expr_lit . lit) , syn :: Expr :: Group (group) => Self :: from_expr (& group . expr) , _ => Err (Error :: unexpected_expr_type (expr)) , } } fn from_value (value : & Lit) -> Result < Self > { let expr_array = syn :: ExprArray :: from_value (value) ?; Self :: from_expr (& syn :: Expr :: Array (expr_array)) } } } ; }
};
}
