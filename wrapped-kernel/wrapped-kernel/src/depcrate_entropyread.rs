// Generated macro for read (function)
macro_rules! Depcrate_entropyread {
() => {
// Module: crate::entropy
// Provides: {"read"}
// Dependencies: {}
# [doc = " Fills `buf` with random data, respecting the options in `flags`."] # [doc = ""] # [doc = " Returns the number of bytes written or `-ENOSYS` if the system does not support"] # [doc = " random data generation."] pub fn read (buf : & mut [u8] , _flags : Flags) -> isize { let pool = & mut * POOL . lock () ; let now = get_timer_ticks () ; let pool = match pool { Some (pool) if now . saturating_sub (pool . last_reseed) <= RESEED_INTERVAL => pool , pool => { if let Some (seed) = seed_entropy () { pool . insert (Pool { rng : ChaCha20Rng :: from_seed (seed) , last_reseed : now , }) } else { return - i32 :: from (Errno :: Nosys) as isize ; } } } ; pool . rng . fill_bytes (buf) ; buf . len () as isize }
};
}
