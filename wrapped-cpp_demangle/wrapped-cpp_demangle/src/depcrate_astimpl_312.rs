// Generated macro for impl_312 (impl)
macro_rules! Depcrate_astimpl_312 {
() => {
// Module: crate::ast
// Provides: {"impl_312"}
// Dependencies: {}
impl < 'a > GetLeafName < 'a > for WellKnownComponent { fn get_leaf_name (& 'a self , _ : & 'a SubstitutionTable) -> Option < LeafName < 'a > > { match * self { WellKnownComponent :: Std => None , _ => Some (LeafName :: WellKnownComponent (self)) , } } }
};
}
