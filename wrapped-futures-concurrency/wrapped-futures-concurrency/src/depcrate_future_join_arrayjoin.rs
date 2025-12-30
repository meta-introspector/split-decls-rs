// Generated macro for Join (struct)
macro_rules! Depcrate_future_join_arrayJoin {
() => {
// Module: crate::future::join::array
// Provides: {"Join"}
// Dependencies: {}
# [doc = " A future which waits for two similarly-typed futures to complete."] # [doc = ""] # [doc = " This `struct` is created by the [`join`] method on the [`Join`] trait. See"] # [doc = " its documentation for more."] # [doc = ""] # [doc = " [`join`]: crate::future::Join::join"] # [doc = " [`Join`]: crate::future::Join"] # [must_use = "futures do nothing unless you `.await` or poll them"] # [pin_project (PinnedDrop)] pub struct Join < Fut , const N : usize > where Fut : Future , { # [doc = " A boolean which holds whether the future has completed"] consumed : bool , # [doc = " The number of futures which are currently still in-flight"] pending : usize , # [doc = " The output data, to be returned after the future completes"] items : OutputArray < < Fut as Future > :: Output , N > , # [doc = " A structure holding the waker passed to the future, and the various"] # [doc = " sub-wakers passed to the contained futures."] wakers : WakerArray < N > , # [doc = " The individual poll state of each future."] state : PollArray < N > , # [pin] # [doc = " The array of futures passed to the structure."] futures : FutureArray < Fut , N > , }
};
}
