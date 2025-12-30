// Generated macro for tests (module)
macro_rules! Depcrate_max_sizetests {
() => {
// Module: crate::max_size
// Provides: {"tests"}
// Dependencies: {}
# [cfg (any (feature = "alloc" , feature = "use-std"))] # [cfg (test)] mod tests { extern crate alloc ; use super :: * ; use alloc :: rc :: Rc ; # [cfg (target_has_atomic = "ptr")] use alloc :: sync :: Arc ; # [test] fn box_max_size () { assert_eq ! (Box ::< u8 >:: POSTCARD_MAX_SIZE , 1) ; assert_eq ! (Box ::< u32 >:: POSTCARD_MAX_SIZE , 5) ; assert_eq ! (Box ::< (u128 , [u8 ; 8]) >:: POSTCARD_MAX_SIZE , 27) ; } # [test] # [cfg (target_has_atomic = "ptr")] fn arc_max_size () { assert_eq ! (Arc ::< u8 >:: POSTCARD_MAX_SIZE , 1) ; assert_eq ! (Arc ::< u32 >:: POSTCARD_MAX_SIZE , 5) ; assert_eq ! (Arc ::< (u128 , [u8 ; 8]) >:: POSTCARD_MAX_SIZE , 27) ; } # [test] fn rc_max_size () { assert_eq ! (Rc ::< u8 >:: POSTCARD_MAX_SIZE , 1) ; assert_eq ! (Rc ::< u32 >:: POSTCARD_MAX_SIZE , 5) ; assert_eq ! (Rc ::< (u128 , [u8 ; 8]) >:: POSTCARD_MAX_SIZE , 27) ; } }
};
}
