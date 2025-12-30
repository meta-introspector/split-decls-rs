// Generated macro for impl_34 (impl)
macro_rules! Depcrateimpl_34 {
() => {
// Module: crate
// Provides: {"impl_34"}
// Dependencies: {}
impl JitterRng < () > { # [doc = " Create a new `JitterRng`. Makes use of `std::time` for a timer, or a"] # [doc = " platform-specific function with higher accuracy if necessary and"] # [doc = " available."] # [doc = ""] # [doc = " During initialization CPU execution timing jitter is measured a few"] # [doc = " hundred times. If this does not pass basic quality tests, an error is"] # [doc = " returned. The test result is cached to make subsequent calls faster."] # [cfg (all (feature = "std" , not (target_arch = "wasm32")))] pub fn new () -> Result < JitterRng < impl Fn () -> u64 + Send + Sync > , TimerError > { if cfg ! (target_arch = "wasm32") { return Err (TimerError :: NoTimer) ; } let mut state = JitterRng :: new_with_timer (platform :: get_nstime) ; let mut rounds = JITTER_ROUNDS . load (Ordering :: Relaxed) as u8 ; if rounds == 0 { rounds = state . test_timer () ? ; JITTER_ROUNDS . store (rounds as usize , Ordering :: Relaxed) ; info ! ("JitterRng: using {} rounds per u64 output" , rounds) ; } state . set_rounds (rounds) ; state . gen_entropy () ; Ok (state) } }
};
}
