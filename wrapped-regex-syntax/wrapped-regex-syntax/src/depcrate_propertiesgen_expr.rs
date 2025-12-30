// Generated macro for gen_expr (function)
macro_rules! Depcrate_propertiesgen_expr {
() => {
// Module: crate::properties
// Provides: {"gen_expr"}
// Dependencies: {}
fn gen_expr < G : Gen > (g : & mut G , depth : u32 , ty : ExprType) -> Expr { use Expr :: * ; let ub = match (depth as usize >= g . size () , ty) { (true , _) => 16 , (false , ExprType :: NoSequences) => 18 , (false , ExprType :: Anything) => 20 , } ; match g . gen_range (1 , ub) { 0 => Empty , 1 => Literal { chars : SmallAscii :: arbitrary (g) . 0 . chars () . collect () , casei : g . gen () , } , 2 => LiteralBytes { bytes : SmallAscii :: arbitrary (g) . 0 . as_bytes () . to_owned () , casei : g . gen () , } , 3 => AnyChar , 4 => AnyCharNoNL , 5 => AnyByte , 6 => AnyByteNoNL , 7 => Class (CharClass :: arbitrary (g)) , 8 => StartLine , 9 => EndLine , 10 => StartText , 11 => EndText , 12 => WordBoundary , 13 => NotWordBoundary , 14 => WordBoundaryAscii , 15 => NotWordBoundaryAscii , 16 => gen_group_expr (g , depth + 1) , 17 => Repeat { e : Box :: new (gen_repeatable_expr (g , depth + 1)) , r : Repeater :: arbitrary (g) , greedy : bool :: arbitrary (g) , } , 18 => { let size = { let s = g . size () ; g . gen_range (2 , s) } ; Concat ((0 .. size) . map (| _ | { gen_expr (g , depth + 1 , ExprType :: NoSequences) }) . collect ()) } 19 => { let size = { let s = g . size () ; g . gen_range (2 , s) } ; Alternate ((0 .. size) . map (| _ | { gen_expr (g , depth + 1 , ExprType :: NoSequences) }) . collect ()) } _ => unreachable ! () } }
};
}
