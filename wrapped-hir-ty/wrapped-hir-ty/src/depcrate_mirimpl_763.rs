// Generated macro for impl_763 (impl)
macro_rules! Depcrate_mirimpl_763 {
() => {
// Module: crate::mir
// Provides: {"impl_763"}
// Dependencies: {}
impl ProjectionId { pub const EMPTY : ProjectionId = ProjectionId (0) ; pub fn is_empty (self) -> bool { self == ProjectionId :: EMPTY } pub fn lookup (self , store : & ProjectionStore) -> & [PlaceElem] { store . id_to_proj . get (& self) . unwrap () } pub fn project (self , projection : PlaceElem , store : & mut ProjectionStore) -> ProjectionId { let mut current = self . lookup (store) . to_vec () ; current . push (projection) ; store . intern (current . into ()) } }
};
}
