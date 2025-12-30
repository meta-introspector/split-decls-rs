// Generated macro for impl_38 (impl)
macro_rules! Depcrate_abiimpl_38 {
() => {
// Module: crate::abi
// Provides: {"impl_38"}
// Dependencies: {}
impl GccType for CastTarget { fn gcc_type < 'gcc > (& self , cx : & CodegenCx < 'gcc , '_ >) -> Type < 'gcc > { let rest_gcc_unit = self . rest . unit . gcc_type (cx) ; let (rest_count , rem_bytes) = if self . rest . unit . size . bytes () == 0 { (0 , 0) } else { (self . rest . total . bytes () / self . rest . unit . size . bytes () , self . rest . total . bytes () % self . rest . unit . size . bytes () ,) } ; if self . prefix . iter () . all (| x | x . is_none ()) { if self . rest . total <= self . rest . unit . size { return rest_gcc_unit ; } if rem_bytes == 0 { return cx . type_array (rest_gcc_unit , rest_count) ; } } let mut args : Vec < _ > = self . prefix . iter () . flat_map (| option_reg | option_reg . map (| reg | reg . gcc_type (cx))) . chain ((0 .. rest_count) . map (| _ | rest_gcc_unit)) . collect () ; if rem_bytes != 0 { assert_eq ! (self . rest . unit . kind , RegKind :: Integer) ; args . push (cx . type_ix (rem_bytes * 8)) ; } cx . type_struct (& args , false) } }
};
}
