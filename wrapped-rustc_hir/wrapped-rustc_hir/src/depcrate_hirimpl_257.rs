// Generated macro for impl_257 (impl)
macro_rules! Depcrate_hirimpl_257 {
() => {
// Module: crate::hir
// Provides: {"impl_257"}
// Dependencies: {}
impl Attribute { pub fn get_normal_item (& self) -> & AttrItem { match & self { Attribute :: Unparsed (normal) => & normal , _ => panic ! ("unexpected parsed attribute") , } } pub fn unwrap_normal_item (self) -> AttrItem { match self { Attribute :: Unparsed (normal) => * normal , _ => panic ! ("unexpected parsed attribute") , } } pub fn value_lit (& self) -> Option < & MetaItemLit > { match & self { Attribute :: Unparsed (n) => match n . as_ref () { AttrItem { args : AttrArgs :: Eq { eq_span : _ , expr } , .. } => Some (expr) , _ => None , } , _ => None , } } pub fn is_parsed_attr (& self) -> bool { match self { Attribute :: Parsed (_) => true , Attribute :: Unparsed (_) => false , } } }
};
}
