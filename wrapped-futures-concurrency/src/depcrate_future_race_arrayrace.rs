// Generated macro for Race (struct)
macro_rules! Depcrate_future_race_arrayRace {
() => {
// Module: crate::future::race::array
// Provides: {"Race"}
// Dependencies: {}
# [doc = " A future which waits for the first future to complete."] # [doc = ""] # [doc = " This `struct` is created by the [`race`] method on the [`Race`] trait. See"] # [doc = " its documentation for more."] # [doc = ""] # [doc = " [`race`]: crate::future::Race::race"] # [doc = " [`Race`]: crate::future::Race"] # [must_use = "futures do nothing unless you `.await` or poll them"] # [pin_project] pub struct Race < Fut , const N : usize > where Fut : Future , { # [pin] futures : [Fut ; N] , indexer : Indexer , done : bool , }
};
}
