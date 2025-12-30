// Generated macro for get_or_init_flags_cache (function)
macro_rules! Depcrate_math_support_feature_detectget_or_init_flags_cache {
() => {
// Module: crate::math::support::feature_detect
// Provides: {"get_or_init_flags_cache"}
// Dependencies: {}
# [doc = " Load flags from an atomic value. If the flags have not yet been initialized, call `init`"] # [doc = " to do so."] # [doc = ""] # [doc = " Note that `init` may run more than once."] # [allow (dead_code)] pub fn get_or_init_flags_cache (cache : & AtomicU32 , init : impl FnOnce () -> Flags) -> Flags { const INITIALIZED : u32 = 1 << 31 ; let mut flags = Flags :: from_bits (cache . load (Ordering :: Relaxed)) ; if ! flags . contains (INITIALIZED) { cold_path () ; flags = init () ; debug_assert ! (! flags . contains (INITIALIZED) , "initialized bit shouldn't be set") ; flags . insert (INITIALIZED) ; cache . store (flags . bits () , Ordering :: Relaxed) ; } flags }
};
}
