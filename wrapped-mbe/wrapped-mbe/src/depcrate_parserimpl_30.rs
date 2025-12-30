// Generated macro for impl_30 (impl)
macro_rules! Depcrate_parserimpl_30 {
() => {
// Module: crate::parser
// Provides: {"impl_30"}
// Dependencies: {}
impl PartialEq for Separator { fn eq (& self , other : & Separator) -> bool { use Separator :: * ; match (self , other) { (Ident (a) , Ident (b)) => a . sym == b . sym , (Literal (a) , Literal (b)) => a . symbol == b . symbol , (Puncts (a) , Puncts (b)) if a . len () == b . len () => { let a_iter = a . iter () . map (| a | a . char) ; let b_iter = b . iter () . map (| b | b . char) ; a_iter . eq (b_iter) } (Lifetime (_ , a) , Lifetime (_ , b)) => a . sym == b . sym , _ => false , } } }
};
}
