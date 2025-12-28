macro_rules! deps {
    () => {
        Flags!();
    };
}

macro_rules! get_or_init_flags_cache {
    () => {
        deps!();
        # [doc = " Load flags from an atomic value. If the flags have not yet been initialized, call `init`"] # [doc = " to do so."] # [doc = ""] # [doc = " Note that `init` may run more than once."] # [allow (dead_code)] pub fn get_or_init_flags_cache (cache : & AtomicU32 , init : impl FnOnce () -> Flags) -> Flags { const INITIALIZED : u32 = 1 << 31 ; let mut flags = Flags :: from_bits (cache . load (Ordering :: Relaxed)) ; if ! flags . contains (INITIALIZED) { cold_path () ; flags = init () ; debug_assert ! (! flags . contains (INITIALIZED) , "initialized bit shouldn't be set") ; flags . insert (INITIALIZED) ; cache . store (flags . bits () , Ordering :: Relaxed) ; } flags }
    };
}

get_or_init_flags_cache!()