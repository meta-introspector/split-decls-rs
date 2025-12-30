// Generated macro for impl_ranged_type_trait (macro)
macro_rules! Depcrate_coord_ranged1d_types_numericimpl_ranged_type_trait {
() => {
// Module: crate::coord::ranged1d::types::numeric
// Provides: {"impl_ranged_type_trait"}
// Dependencies: {}
macro_rules ! impl_ranged_type_trait { ($ value : ty , $ coord : ident) => { impl AsRangedCoord for Range <$ value > { type CoordDescType = $ coord ; type Value = $ value ; } } ; }
};
}
