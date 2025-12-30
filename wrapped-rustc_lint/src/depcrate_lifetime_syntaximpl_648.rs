// Generated macro for impl_648 (impl)
macro_rules! Depcrate_lifetime_syntaximpl_648 {
() => {
// Module: crate::lifetime_syntax
// Provides: {"impl_648"}
// Dependencies: {}
impl < T > LifetimeSyntaxCategories < Vec < T > > { pub fn len (& self) -> LifetimeSyntaxCategories < usize > { LifetimeSyntaxCategories { hidden : self . hidden . len () , elided : self . elided . len () , named : self . named . len () , } } pub fn iter_unnamed (& self) -> impl Iterator < Item = & T > { let Self { hidden , elided , named : _ } = self ; [hidden . iter () , elided . iter ()] . into_iter () . flatten () } }
};
}
