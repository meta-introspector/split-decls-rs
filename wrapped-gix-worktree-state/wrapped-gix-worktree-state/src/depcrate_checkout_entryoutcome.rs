// Generated macro for Outcome (enum)
macro_rules! Depcrate_checkout_entryOutcome {
() => {
// Module: crate::checkout::entry
// Provides: {"Outcome"}
// Dependencies: {}
pub enum Outcome < 'a > { # [doc = " The file was written."] Written { # [doc = " The amount of written bytes."] bytes : usize , } , # [doc = " The will be ready later."] Delayed (DelayedFilteredStream < 'a >) , }
};
}
