// Generated macro for impl_134 (impl)
macro_rules! Depcrate_astimpl_134 {
() => {
// Module: crate::ast
// Provides: {"impl_134"}
// Dependencies: {}
impl Extend < TypeModifier > for TypeModifiers { fn extend < T : IntoIterator < Item = TypeModifier > > (& mut self , iter : T) { for modifier in iter { self . wrap (modifier) ; } } }
};
}
