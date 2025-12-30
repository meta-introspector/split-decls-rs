// Generated macro for gen_repeatable_expr (function)
macro_rules! Depcrate_propertiesgen_repeatable_expr {
() => {
// Module: crate::properties
// Provides: {"gen_repeatable_expr"}
// Dependencies: {}
fn gen_repeatable_expr < G : Gen > (g : & mut G , depth : u32) -> Expr { use Expr :: * ; match g . gen_range (1 , 10) { 0 => Empty , 1 => Literal { chars : vec ! [Arbitrary :: arbitrary (g)] , casei : g . gen () , } , 2 => LiteralBytes { bytes : vec ! [Arbitrary :: arbitrary (g)] , casei : g . gen () , } , 3 => AnyChar , 4 => AnyCharNoNL , 5 => AnyByte , 6 => AnyByteNoNL , 7 => Class (CharClass :: arbitrary (g)) , 8 => ClassBytes (ByteClass :: arbitrary (g)) , 9 => gen_group_expr (g , depth + 1) , _ => unreachable ! () , } }
};
}
