// Generated macro for RaceOk (struct)
macro_rules! Depcrate_future_race_ok_vecRaceOk {
() => {
// Module: crate::future::race_ok::vec
// Provides: {"RaceOk"}
// Dependencies: {}
# [doc = " A future which waits for the first successful future to complete."] # [doc = ""] # [doc = " This `struct` is created by the [`race_ok`] method on the [`RaceOk`] trait. See"] # [doc = " its documentation for more."] # [doc = ""] # [doc = " [`race_ok`]: crate::future::RaceOk::race_ok"] # [doc = " [`RaceOk`]: crate::future::RaceOk"] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct RaceOk < Fut , T , E > where Fut : Future < Output = Result < T , E > > , { elems : Pin < Box < [MaybeDone < Fut >] > > , }
};
}
