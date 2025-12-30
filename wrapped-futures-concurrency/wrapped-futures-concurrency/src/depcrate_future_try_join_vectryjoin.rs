// Generated macro for TryJoin (struct)
macro_rules! Depcrate_future_try_join_vecTryJoin {
() => {
// Module: crate::future::try_join::vec
// Provides: {"TryJoin"}
// Dependencies: {}
# [doc = " A future which waits for all futures to complete successfully, or abort early on error."] # [doc = ""] # [doc = " This `struct` is created by the [`try_join`] method on the [`TryJoin`] trait. See"] # [doc = " its documentation for more."] # [doc = ""] # [doc = " [`try_join`]: crate::future::TryJoin::try_join"] # [doc = " [`TryJoin`]: crate::future::TryJoin"] # [must_use = "futures do nothing unless you `.await` or poll them"] # [pin_project (PinnedDrop)] pub struct TryJoin < Fut , T , E > where Fut : Future < Output = Result < T , E > > , { # [doc = " A boolean which holds whether the future has completed"] consumed : bool , # [doc = " The number of futures which are currently still in-flight"] pending : usize , # [doc = " The output data, to be returned after the future completes"] items : OutputVec < T > , # [doc = " A structure holding the waker passed to the future, and the various"] # [doc = " sub-wakers passed to the contained futures."] wakers : WakerVec , # [doc = " The individual poll state of each future."] state : PollVec , # [pin] # [doc = " The array of futures passed to the structure."] futures : FutureVec < Fut > , }
};
}
