// Generated macro for tests (module)
macro_rules! Depcrate_index_settests {
() => {
// Module: crate::index_set
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use static_assertions :: assert_not_impl_any ; use super :: { BuildHasherDefault , IndexSet } ; assert_not_impl_any ! (IndexSet <* const () , BuildHasherDefault < () >, 4 >: Send) ; # [test] # [cfg (feature = "zeroize")] fn test_index_set_zeroize () { use zeroize :: Zeroize ; let mut set : IndexSet < u8 , BuildHasherDefault < hash32 :: FnvHasher > , 8 > = IndexSet :: new () ; for i in 1 ..= 8 { set . insert (i) . unwrap () ; } assert_eq ! (set . len () , 8) ; assert ! (set . contains (& 8)) ; set . zeroize () ; assert_eq ! (set . len () , 0) ; assert ! (set . is_empty ()) ; set . insert (1) . unwrap () ; assert_eq ! (set . len () , 1) ; assert ! (set . contains (& 1)) ; } }
};
}
