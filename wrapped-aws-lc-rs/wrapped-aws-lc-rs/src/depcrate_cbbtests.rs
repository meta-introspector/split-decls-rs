// Generated macro for tests (module)
macro_rules! Depcrate_cbbtests {
() => {
// Module: crate::cbb
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: LcCBB ; use crate :: aws_lc :: CBB_add_asn1_bool ; # [test] fn dynamic_vec () { let mut cbb = LcCBB :: new (4) ; assert_eq ! (1 , unsafe { CBB_add_asn1_bool (cbb . as_mut_ptr () , 1) }) ; let vec = cbb . into_vec () . expect ("be copied to buffer") ; assert_eq ! (vec . as_slice () , & [1 , 1 , 255]) ; } # [test] fn dynamic_buffer_grows () { let mut cbb = LcCBB :: new (1) ; assert_eq ! (1 , unsafe { CBB_add_asn1_bool (cbb . as_mut_ptr () , 1) }) ; let vec = cbb . into_vec () . expect ("be copied to buffer") ; assert_eq ! (vec . as_slice () , & [1 , 1 , 255]) ; } # [test] fn fixed_buffer () { let mut buffer = [0u8 ; 4] ; let mut cbb = LcCBB :: new_from_slice (& mut buffer) ; assert_eq ! (1 , unsafe { CBB_add_asn1_bool (cbb . as_mut_ptr () , 1) }) ; let out_len = cbb . finish () . expect ("cbb finishable") ; assert_eq ! (& buffer [.. out_len] , & [1 , 1 , 255]) ; } # [test] fn fixed_buffer_no_growth () { let mut buffer = [0u8 ; 1] ; let mut cbb = LcCBB :: new_from_slice (& mut buffer) ; assert_ne ! (1 , unsafe { CBB_add_asn1_bool (cbb . as_mut_ptr () , 1) }) ; } }
};
}
