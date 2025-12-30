// Generated macro for impl_53 (impl)
macro_rules! Depcrate_store_impls_dynamic_iterimpl_53 {
() => {
// Module: crate::store_impls::dynamic::iter
// Provides: {"impl_53"}
// Dependencies: {}
impl < S > super :: Handle < S > where S : Deref < Target = super :: Store > + Clone , { # [doc = " Return an iterator over all, _possibly duplicate_, objects, first the ones in all packs of all linked databases (via alternates),"] # [doc = " followed by all loose objects."] pub fn iter (& self) -> Result < AllObjects , dynamic :: load_index :: Error > { AllObjects :: new (self . store_ref ()) } }
};
}
