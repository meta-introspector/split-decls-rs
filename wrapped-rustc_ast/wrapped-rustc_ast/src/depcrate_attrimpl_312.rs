// Generated macro for impl_312 (impl)
macro_rules! Depcrate_attrimpl_312 {
() => {
// Module: crate::attr
// Provides: {"impl_312"}
// Dependencies: {}
impl Attribute { pub fn get_normal_item (& self) -> & AttrItem { match & self . kind { AttrKind :: Normal (normal) => & normal . item , AttrKind :: DocComment (..) => panic ! ("unexpected doc comment") , } } pub fn unwrap_normal_item (self) -> AttrItem { match self . kind { AttrKind :: Normal (normal) => normal . item , AttrKind :: DocComment (..) => panic ! ("unexpected doc comment") , } } }
};
}
