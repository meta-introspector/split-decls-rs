// Generated macro for gen_group_expr (function)
macro_rules! Depcrate_propertiesgen_group_expr {
() => {
// Module: crate::properties
// Provides: {"gen_group_expr"}
// Dependencies: {}
fn gen_group_expr < G : Gen > (g : & mut G , depth : u32) -> Expr { let (i , name) = if g . gen () { (None , None) } else { (Some (0) , if g . gen () { Some (SmallAscii :: arbitrary (g) . 0) } else { None }) } ; Expr :: Group { e : Box :: new (gen_expr (g , depth + 1 , ExprType :: Anything)) , i : i , name : name , } }
};
}
