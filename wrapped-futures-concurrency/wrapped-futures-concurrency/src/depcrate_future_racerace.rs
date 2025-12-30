// Generated macro for Race (trait)
macro_rules! Depcrate_future_raceRace {
() => {
// Module: crate::future::race
// Provides: {"Race"}
// Dependencies: {}
# [doc = " Wait for the first future to complete."] # [doc = ""] # [doc = " Awaits multiple future at once, returning as soon as one completes. The"] # [doc = " other futures are cancelled."] pub trait Race { # [doc = " The resulting output type."] type Output ; # [doc = " Which kind of future are we turning this into?"] type Future : Future < Output = Self :: Output > ; # [doc = " Wait for the first future to complete."] # [doc = ""] # [doc = " Awaits multiple futures at once, returning as soon as one completes. The"] # [doc = " other futures are cancelled."] # [doc = ""] # [doc = " This function returns a new future which polls all futures concurrently."] fn race (self) -> Self :: Future ; }
};
}
