// Generated macro for hard_fault (function)
macro_rules! Depcratehard_fault {
() => {
// Module: crate
// Provides: {"hard_fault"}
// Dependencies: {}
# [doc = " Trigger a `HardFault` via `udf` instruction."] # [doc = ""] # [doc = " This function may be used to as `defmt::panic_handler` to avoid double prints."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #[defmt::panic_handler]"] # [doc = " fn panic() -> ! {"] # [doc = "     panic_probe::hard_fault();"] # [doc = " }"] # [doc = " ```"] # [cfg (target_os = "none")] pub fn hard_fault () -> ! { # [cfg (not (any (armv6m , armv8m_base)))] { const SHCSR : * mut u32 = 0xE000ED24usize as _ ; const USGFAULTENA : usize = 18 ; unsafe { let mut shcsr = core :: ptr :: read_volatile (SHCSR) ; shcsr &= ! (1 << USGFAULTENA) ; core :: ptr :: write_volatile (SHCSR , shcsr) ; } } cortex_m :: asm :: udf () ; }
};
}
