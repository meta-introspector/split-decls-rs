// Generated macro for impl_17 (impl)
macro_rules! Depcrate_arbitrary_in_macro_rulesimpl_17 {
() => {
// Module: crate::arbitrary_in_macro_rules
// Provides: {"impl_17"}
// Dependencies: {}
impl ItemAttrs for Item { fn attrs (& self) -> & [syn :: Attribute] { match self { Item :: Struct (s) => & s . attrs , Item :: Enum (e) => & e . attrs , _ => & [] , } } fn attrs_mut (& mut self) -> & mut Vec < syn :: Attribute > { match self { Item :: Struct (s) => & mut s . attrs , Item :: Enum (e) => & mut e . attrs , _ => panic ! ("Unsupported item type") , } } }
};
}
