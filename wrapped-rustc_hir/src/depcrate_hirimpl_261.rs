// Generated macro for impl_261 (impl)
macro_rules! Depcrate_hirimpl_261 {
() => {
// Module: crate::hir
// Provides: {"impl_261"}
// Dependencies: {}
impl < 'tcx > AttributeMap < 'tcx > { pub const EMPTY : & 'static AttributeMap < 'static > = & AttributeMap { map : SortedMap :: new () , opt_hash : Some (Fingerprint :: ZERO) , define_opaque : None , } ; # [inline] pub fn get (& self , id : ItemLocalId) -> & 'tcx [Attribute] { self . map . get (& id) . copied () . unwrap_or (& []) } }
};
}
