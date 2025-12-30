// Generated macro for impl_68 (impl)
macro_rules! Depcrate_deimpl_68 {
() => {
// Module: crate::de
// Provides: {"impl_68"}
// Dependencies: {}
impl < T > BorshDeserialize for BTreeSet < T > where T : BorshDeserialize + Ord , { # [inline] fn deserialize_reader < R : Read > (reader : & mut R) -> Result < Self > { let vec = < Vec < T > > :: deserialize_reader (reader) ? ; # [cfg (feature = "de_strict_order")] for pair in vec . windows (2) { let [a , b] = pair else { unreachable ! ("`windows` always return a slice of length 2 or nothing") ; } ; let cmp_result = a . cmp (b) . is_lt () ; if ! cmp_result { return Err (Error :: new (ErrorKind :: InvalidData , ERROR_WRONG_ORDER_OF_KEYS ,)) ; } } Ok (vec . into_iter () . collect :: < BTreeSet < T > > ()) } }
};
}
