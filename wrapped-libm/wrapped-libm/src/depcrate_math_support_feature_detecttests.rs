// Generated macro for tests (module)
macro_rules! Depcrate_math_support_feature_detecttests {
() => {
// Module: crate::math::support::feature_detect
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn unique_masks () { unique_masks ! { u32 , V0 , V1 , V2 , } assert_eq ! (V0 , 1u32 << 0) ; assert_eq ! (V1 , 1u32 << 1) ; assert_eq ! (V2 , 1u32 << 2) ; assert_eq ! (ALL , [V0 , V1 , V2]) ; assert_eq ! (NAMES , ["V0" , "V1" , "V2"]) ; } # [test] fn flag_cache_is_used () { static CACHE : AtomicU32 = AtomicU32 :: new (0) ; let mut f1 = Flags :: from_bits (0x1) ; let f2 = Flags :: from_bits (0x2) ; let r1 = get_or_init_flags_cache (& CACHE , | | f1) ; let r2 = get_or_init_flags_cache (& CACHE , | | f2) ; f1 . insert (1 << 31) ; assert_eq ! (r1 , f1) ; assert_eq ! (r2 , f1) ; } # [test] fn select_cache_is_used () { static CALLED : AtomicU32 = AtomicU32 :: new (0) ; fn inner () { fn nop () { } select_once ! { sig : fn () -> () , init : || { CALLED . fetch_add (1 , Ordering :: Relaxed) ; nop } , call : | fn_ptr : Func | unsafe { fn_ptr () } , } } inner () ; assert_eq ! (CALLED . load (Ordering :: Relaxed) , 1) ; inner () ; assert_eq ! (CALLED . load (Ordering :: Relaxed) , 1) ; } }
};
}
