use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Hasher {
    /// Create a new `Hasher`.
    ///
    /// This will perform a CPU feature detection at runtime to select the most
    /// optimal implementation for the current processor architecture.
    pub fn new() -> Self {
        Self::new_with_initial(DEFAULT_INIT_STATE)
    }
    /// Create a new `Hasher` with an initial CRC32 state.
    ///
    /// This works just like `Hasher::new`, except that it allows for an initial
    /// CRC32 state to be passed in.
    pub fn new_with_initial(init: u32) -> Self {
        Self::new_with_initial_len(init, 0)
    }
    /// Create a new `Hasher` with an initial CRC32 state.
    ///
    /// As `new_with_initial`, but also accepts a length (in bytes). The
    /// resulting object can then be used with `combine` to compute `crc(a ||
    /// b)` from `crc(a)`, `crc(b)`, and `len(b)`.
    pub fn new_with_initial_len(init: u32, amount: u64) -> Self {
        Self::internal_new_specialized(init, amount)
            .unwrap_or_else(|| Self::internal_new_baseline(init, amount))
    }
    #[doc(hidden)]
    pub fn internal_new_baseline(init: u32, amount: u64) -> Self {
        Hasher {
            amount,
            state: State::Baseline(baseline::State::new(init)),
        }
    }
    #[doc(hidden)]
    pub fn internal_new_specialized(init: u32, amount: u64) -> Option<Self> {
        {
            if let Some(state) = specialized::State::new(init) {
                return Some(Hasher {
                    amount,
                    state: State::Specialized(state),
                });
            }
        }
        None
    }
    /// Process the given byte slice and update the hash state.
    pub fn update(&mut self, buf: &[u8]) {
        self.amount += buf.len() as u64;
        match self.state {
            State::Baseline(ref mut state) => state.update(buf),
            State::Specialized(ref mut state) => state.update(buf),
        }
    }
    /// Finalize the hash state and return the computed CRC32 value.
    pub fn finalize(self) -> u32 {
        match self.state {
            State::Baseline(state) => state.finalize(),
            State::Specialized(state) => state.finalize(),
        }
    }
    /// Reset the hash state.
    pub fn reset(&mut self) {
        self.amount = 0;
        match self.state {
            State::Baseline(ref mut state) => state.reset(),
            State::Specialized(ref mut state) => state.reset(),
        }
    }
    /// Combine the hash state with the hash state for the subsequent block of bytes.
    pub fn combine(&mut self, other: &Self) {
        self.amount += other.amount;
        let other_crc = other.clone().finalize();
        match self.state {
            State::Baseline(ref mut state) => state.combine(other_crc, other.amount),
            State::Specialized(ref mut state) => state.combine(other_crc, other.amount),
        }
    }
}
