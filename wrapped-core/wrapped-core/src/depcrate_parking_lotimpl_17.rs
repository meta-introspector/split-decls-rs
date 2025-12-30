// Generated macro for impl_17 (impl)
macro_rules! Depcrate_parking_lotimpl_17 {
() => {
// Module: crate::parking_lot
// Provides: {"impl_17"}
// Dependencies: {}
impl FairTimeout { # [inline] fn new (timeout : TimeoutInstant , seed : u32) -> FairTimeout { FairTimeout { timeout , seed } } # [inline] fn should_timeout (& mut self) -> bool { let now = TimeoutInstant :: now () ; if now > self . timeout { let nanos = self . gen_u32 () % 1_000_000 ; self . timeout = now + Duration :: new (0 , nanos) ; true } else { false } } fn gen_u32 (& mut self) -> u32 { self . seed ^= self . seed << 13 ; self . seed ^= self . seed >> 17 ; self . seed ^= self . seed << 5 ; self . seed } }
};
}
