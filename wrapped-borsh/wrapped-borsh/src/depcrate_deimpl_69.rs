// Generated macro for impl_69 (impl)
macro_rules! Depcrate_deimpl_69 {
() => {
// Module: crate::de
// Provides: {"impl_69"}
// Dependencies: {}
impl < K , V > BorshDeserialize for BTreeMap < K , V > where K : BorshDeserialize + Ord , V : BorshDeserialize , { # [inline] fn deserialize_reader < R : Read > (reader : & mut R) -> Result < Self > { check_zst :: < K > () ? ; let vec = < Vec < (K , V) > > :: deserialize_reader (reader) ? ; # [cfg (feature = "de_strict_order")] for pair in vec . windows (2) { let [(a_k , _a_v) , (b_k , _b_v)] = pair else { unreachable ! ("`windows` always return a slice of length 2 or nothing") ; } ; let cmp_result = a_k . cmp (b_k) . is_lt () ; if ! cmp_result { return Err (Error :: new (ErrorKind :: InvalidData , ERROR_WRONG_ORDER_OF_KEYS ,)) ; } } Ok (vec . into_iter () . collect :: < BTreeMap < K , V > > ()) } }
};
}
