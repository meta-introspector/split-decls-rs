// Generated macro for Join (struct)
macro_rules! Depcrate_future_join_vecJoin {
() => {
// Module: crate::future::join::vec
// Provides: {"Join"}
// Dependencies: {}
# [doc = " A future which waits for multiple futures to complete."] # [doc = ""] # [doc = " This `struct` is created by the [`join`] method on the [`Join`] trait. See"] # [doc = " its documentation for more."] # [doc = ""] # [doc = " [`join`]: crate::future::Join::join"] # [doc = " [`Join`]: crate::future::Join"] # [must_use = "futures do nothing unless you `.await` or poll them"] # [pin_project (PinnedDrop)] pub struct Join < Fut > where Fut : Future , { consumed : bool , pending : usize , items : OutputVec < < Fut as Future > :: Output > , wakers : WakerVec , state : PollVec , # [pin] futures : FutureVec < Fut > , }
};
}
