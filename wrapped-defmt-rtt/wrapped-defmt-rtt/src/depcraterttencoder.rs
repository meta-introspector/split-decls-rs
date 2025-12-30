// Generated macro for RttEncoder (struct)
macro_rules! DepcrateRttEncoder {
() => {
// Module: crate
// Provides: {"RttEncoder"}
// Dependencies: {}
struct RttEncoder { # [doc = " A boolean lock"] # [doc = ""] # [doc = " Is `true` when `acquire` has been called and we have exclusive access to"] # [doc = " the rest of this structure."] taken : AtomicBool , # [doc = " We need to remember this to exit a critical section"] cs_restore : UnsafeCell < critical_section :: RestoreState > , # [doc = " A defmt::Encoder for encoding frames"] encoder : UnsafeCell < defmt :: Encoder > , }
};
}
