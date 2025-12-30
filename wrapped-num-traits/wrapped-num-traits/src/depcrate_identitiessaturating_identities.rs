// Generated macro for saturating_identities (function)
macro_rules! Depcrate_identitiessaturating_identities {
() => {
// Module: crate::identities
// Provides: {"saturating_identities"}
// Dependencies: {}
# [test] # [cfg (has_num_saturating)] fn saturating_identities () { macro_rules ! test_saturating_identities { ($ ($ t : ty) +) => { $ (assert_eq ! (zero ::<$ t > () , zero ::< Saturating <$ t >> () . 0) ; assert_eq ! (one ::<$ t > () , one ::< Saturating <$ t >> () . 0) ; assert_eq ! ((0 as $ t) . is_zero () , Saturating (0 as $ t) . is_zero ()) ; assert_eq ! ((1 as $ t) . is_zero () , Saturating (1 as $ t) . is_zero ()) ;) + } ; } test_saturating_identities ! (isize i8 i16 i32 i64 usize u8 u16 u32 u64) ; }
};
}
