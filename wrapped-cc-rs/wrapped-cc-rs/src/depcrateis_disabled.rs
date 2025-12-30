// Generated macro for is_disabled (function)
macro_rules! Depcrateis_disabled {
() => {
// Module: crate
// Provides: {"is_disabled"}
// Dependencies: {}
# [doc = " Returns true if `cc` has been disabled by `CC_FORCE_DISABLE`."] fn is_disabled () -> bool { static CACHE : AtomicU8 = AtomicU8 :: new (0) ; let val = CACHE . load (Relaxed) ; # [allow (clippy :: disallowed_methods)] fn compute_is_disabled () -> bool { match std :: env :: var_os ("CC_FORCE_DISABLE") { None => false , Some (v) => & * v != "0" && & * v != "false" && & * v != "no" , } } match val { 2 => true , 1 => false , 0 => { let truth = compute_is_disabled () ; let encoded_truth = if truth { 2u8 } else { 1 } ; let _ = CACHE . compare_exchange (0 , encoded_truth , Relaxed , Relaxed) ; truth } _ => unreachable ! () , } }
};
}
