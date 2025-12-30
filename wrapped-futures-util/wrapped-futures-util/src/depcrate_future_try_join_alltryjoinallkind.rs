// Generated macro for TryJoinAllKind (enum)
macro_rules! Depcrate_future_try_join_allTryJoinAllKind {
() => {
// Module: crate::future::try_join_all
// Provides: {"TryJoinAllKind"}
// Dependencies: {}
enum TryJoinAllKind < F > where F : TryFuture , { Small { elems : Pin < Box < [TryMaybeDone < IntoFuture < F > >] > > , } , # [cfg_attr (target_os = "none" , cfg (target_has_atomic = "ptr"))] Big { fut : TryCollect < FuturesOrdered < IntoFuture < F > > , Vec < F :: Ok > > , } , }
};
}
