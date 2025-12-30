// Generated macro for const_cmp_bytes (function)
macro_rules! Depcrate_mapconst_cmp_bytes {
() => {
// Module: crate::map
// Provides: {"const_cmp_bytes"}
// Dependencies: {}
const fn const_cmp_bytes (a : & [u8] , b : & [u8]) -> Ordering { let (max , default) = if a . len () == b . len () { (a . len () , Ordering :: Equal) } else if a . len () < b . len () { (a . len () , Ordering :: Less) } else { (b . len () , Ordering :: Greater) } ; let mut i = 0 ; # [expect (clippy :: indexing_slicing)] while i < max { if a [i] == b [i] { i += 1 ; continue ; } else if a [i] < b [i] { return Ordering :: Less ; } else { return Ordering :: Greater ; } } default }
};
}
