// Generated macro for impl_2934 (impl)
macro_rules! Depcrate_pg_types_floats_quickcheck_implsimpl_2934 {
() => {
// Module: crate::pg::types::floats::quickcheck_impls
// Provides: {"impl_2934"}
// Dependencies: {}
impl Arbitrary for PgNumeric { fn arbitrary (g : & mut Gen) -> Self { let mut variant = Option :: < bool > :: arbitrary (g) ; let mut weight = - 1 ; while weight < 0 { weight = i16 :: arbitrary (g) ; } let scale = u16 :: arbitrary (g) & SCALE_MASK ; let digits = gen_vec_of_appropriate_length_valid_digits (g , weight as u16 , scale) ; if digits . is_empty () { weight = 0 ; variant = Some (true) ; } match variant { Some (true) => PgNumeric :: Positive { digits : digits , weight : weight , scale : scale , } , Some (false) => PgNumeric :: Negative { digits : digits , weight : weight , scale : scale , } , None => PgNumeric :: NaN , } } }
};
}
