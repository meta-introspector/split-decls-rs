// Generated macro for Race (struct)
macro_rules! Depcrate_future_race_vecRace {
() => {
// Module: crate::future::race::vec
// Provides: {"Race"}
// Dependencies: {}
# [doc = " A future which waits for the first future to complete."] # [doc = ""] # [doc = " This `struct` is created by the [`race`] method on the [`Race`] trait. See"] # [doc = " its documentation for more."] # [doc = ""] # [doc = " [`race`]: crate::future::Race::race"] # [doc = " [`Race`]: crate::future::Race"] # [must_use = "futures do nothing unless you `.await` or poll them"] # [pin_project] pub struct Race < Fut > where Fut : Future , { # [pin] futures : Vec < Fut > , indexer : Indexer , done : bool , }
};
}
