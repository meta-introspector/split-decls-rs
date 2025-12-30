// Generated macro for impl_765 (impl)
macro_rules! Depcrate_mirimpl_765 {
() => {
// Module: crate::mir
// Provides: {"impl_765"}
// Dependencies: {}
impl Place { fn is_parent (& self , child : & Place , store : & ProjectionStore) -> bool { self . local == child . local && child . projection . lookup (store) . starts_with (self . projection . lookup (store)) } # [doc = " The place itself is not included"] fn iterate_over_parents < 'a > (& 'a self , store : & 'a ProjectionStore ,) -> impl Iterator < Item = Place > + 'a { let projection = self . projection . lookup (store) ; (0 .. projection . len ()) . map (| x | & projection [0 .. x]) . filter_map (move | x | { Some (Place { local : self . local , projection : store . intern_if_exist (x) ? }) }) } fn project (& self , projection : PlaceElem , store : & mut ProjectionStore) -> Place { Place { local : self . local , projection : self . projection . project (projection , store) } } }
};
}
