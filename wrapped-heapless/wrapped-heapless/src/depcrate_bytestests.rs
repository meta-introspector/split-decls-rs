// Generated macro for tests (module)
macro_rules! Depcrate_bytestests {
() => {
// Module: crate::bytes
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: { Vec , VecView } ; use bytes :: BufMut ; # [test] # [should_panic] fn buf_mut_advance_mut_out_of_bounds () { let mut vec : Vec < u8 , 8 > = Vec :: new () ; unsafe { vec . advance_mut (9) } ; } # [test] fn buf_mut_remaining_mut () { let mut vec : Vec < u8 , 8 > = Vec :: new () ; assert_eq ! (vec . remaining_mut () , 8) ; vec . push (42) . unwrap () ; assert_eq ! (vec . remaining_mut () , 7) ; } # [test] fn buf_mut_chunk_mut () { let mut vec : Vec < u8 , 8 > = Vec :: new () ; assert_eq ! (vec . chunk_mut () . len () , 8) ; unsafe { vec . advance_mut (1) } ; assert_eq ! (vec . chunk_mut () . len () , 7) ; } # [test] # [should_panic] fn buf_mut_advance_mut_out_of_bounds_view () { let vec : & mut VecView < u8 , u8 > = & mut Vec :: < u8 , 8 , u8 > :: new () ; unsafe { vec . advance_mut (9) } ; } # [test] fn buf_mut_remaining_mut_view () { let vec : & mut VecView < u8 , u8 > = & mut Vec :: < u8 , 8 , u8 > :: new () ; assert_eq ! (vec . remaining_mut () , 8) ; vec . push (42) . unwrap () ; assert_eq ! (vec . remaining_mut () , 7) ; } # [test] fn buf_mut_chunk_mut_view () { let vec : & mut VecView < u8 , u8 > = & mut Vec :: < u8 , 8 , u8 > :: new () ; assert_eq ! (vec . chunk_mut () . len () , 8) ; unsafe { vec . advance_mut (1) } ; assert_eq ! (vec . chunk_mut () . len () , 7) ; } }
};
}
