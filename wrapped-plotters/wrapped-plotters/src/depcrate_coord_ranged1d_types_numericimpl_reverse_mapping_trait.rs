// Generated macro for impl_reverse_mapping_trait (macro)
macro_rules! Depcrate_coord_ranged1d_types_numericimpl_reverse_mapping_trait {
() => {
// Module: crate::coord::ranged1d::types::numeric
// Provides: {"impl_reverse_mapping_trait"}
// Dependencies: {}
macro_rules ! impl_reverse_mapping_trait { ($ type : ty , $ name : ident) => { impl ReversibleRanged for $ name { fn unmap (& self , p : i32 , (min , max) : (i32 , i32)) -> Option <$ type > { if p < min . min (max) || p > max . max (min) || min == max { return None ; } let logical_offset = f64 :: from (p - min) / f64 :: from (max - min) ; return Some (((self . 1 - self . 0) as f64 * logical_offset + self . 0 as f64) as $ type) ; } } } ; }
};
}
