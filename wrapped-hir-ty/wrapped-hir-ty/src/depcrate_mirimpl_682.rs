// Generated macro for impl_682 (impl)
macro_rules! Depcrate_mirimpl_682 {
() => {
// Module: crate::mir
// Provides: {"impl_682"}
// Dependencies: {}
impl ProjectionId { pub const EMPTY : ProjectionId = ProjectionId (0) ; pub fn is_empty (self) -> bool { self == ProjectionId :: EMPTY } pub fn lookup < 'a , 'db > (self , store : & 'a ProjectionStore < 'db >) -> & 'a [PlaceElem < 'db >] { store . id_to_proj . get (& self) . unwrap () } pub fn project < 'db > (self , projection : PlaceElem < 'db > , store : & mut ProjectionStore < 'db > ,) -> ProjectionId { let mut current = self . lookup (store) . to_vec () ; current . push (projection) ; store . intern (current . into ()) } }
};
}
