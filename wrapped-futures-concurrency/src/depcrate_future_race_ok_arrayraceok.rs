// Generated macro for RaceOk (struct)
macro_rules! Depcrate_future_race_ok_arrayRaceOk {
() => {
// Module: crate::future::race_ok::array
// Provides: {"RaceOk"}
// Dependencies: {}
# [doc = " A future which waits for the first successful future to complete."] # [doc = ""] # [doc = " This `struct` is created by the [`race_ok`] method on the [`RaceOk`] trait. See"] # [doc = " its documentation for more."] # [doc = ""] # [doc = " [`race_ok`]: crate::future::RaceOk::race_ok"] # [doc = " [`RaceOk`]: crate::future::RaceOk"] # [must_use = "futures do nothing unless you `.await` or poll them"] # [pin_project (PinnedDrop)] pub struct RaceOk < Fut , T , E , const N : usize > where Fut : Future < Output = Result < T , E > > , { # [pin] futures : [Fut ; N] , errors : [MaybeUninit < E > ; N] , error_states : PollArray < N > , completed : usize , }
};
}
