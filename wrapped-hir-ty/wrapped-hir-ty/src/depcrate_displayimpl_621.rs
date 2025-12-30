// Generated macro for impl_621 (impl)
macro_rules! Depcrate_displayimpl_621 {
() => {
// Module: crate::display
// Provides: {"impl_621"}
// Dependencies: {}
impl SizedByDefault { fn is_sized_trait (self , trait_ : TraitId , db : & dyn DefDatabase) -> bool { match self { Self :: NotSized => false , Self :: Sized { anchor } => { let sized_trait = LangItem :: Sized . resolve_trait (db , anchor) ; Some (trait_) == sized_trait } } } }
};
}
