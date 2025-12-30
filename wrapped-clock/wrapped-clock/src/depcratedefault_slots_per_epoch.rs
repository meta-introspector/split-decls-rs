// Generated macro for DEFAULT_SLOTS_PER_EPOCH (const)
macro_rules! DepcrateDEFAULT_SLOTS_PER_EPOCH {
() => {
// Module: crate
// Provides: {"DEFAULT_SLOTS_PER_EPOCH"}
// Dependencies: {}
# [doc = " The number of slots per epoch after initial network warmup."] # [doc = ""] # [doc = " 1 Epoch ~= 2 days."] pub const DEFAULT_SLOTS_PER_EPOCH : u64 = 2 * TICKS_PER_DAY / DEFAULT_TICKS_PER_SLOT ;
};
}
