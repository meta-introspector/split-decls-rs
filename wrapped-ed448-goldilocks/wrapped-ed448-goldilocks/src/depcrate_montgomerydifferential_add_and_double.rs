// Generated macro for differential_add_and_double (function)
macro_rules! Depcrate_montgomerydifferential_add_and_double {
() => {
// Module: crate::montgomery
// Provides: {"differential_add_and_double"}
// Dependencies: {}
fn differential_add_and_double (P : & mut ProjectiveMontgomeryPoint , Q : & mut ProjectiveMontgomeryPoint , affine_PmQ : & FieldElement ,) { let t0 = P . U + P . W ; let t1 = P . U - P . W ; let t2 = Q . U + Q . W ; let t3 = Q . U - Q . W ; let t4 = t0 . square () ; let t5 = t1 . square () ; let t6 = t4 - t5 ; let t7 = t0 * t3 ; let t8 = t1 * t2 ; let t9 = t7 + t8 ; let t10 = t7 - t8 ; let t11 = t9 . square () ; let t12 = t10 . square () ; let t13 = FieldElement :: A_PLUS_TWO_OVER_FOUR * t6 ; let t14 = t4 * t5 ; let t15 = t13 + t5 ; let t16 = t6 * t15 ; let t17 = * affine_PmQ * t12 ; let t18 = t11 ; P . U = t14 ; P . W = t16 ; Q . U = t18 ; Q . W = t17 ; }
};
}
