// Generated macro for JoinAllKind (enum)
macro_rules! Depcrate_future_join_allJoinAllKind {
() => {
// Module: crate::future::join_all
// Provides: {"JoinAllKind"}
// Dependencies: {}
enum JoinAllKind < F > where F : Future , { Small { elems : Pin < Box < [MaybeDone < F >] > > , } , # [cfg_attr (target_os = "none" , cfg (target_has_atomic = "ptr"))] Big { fut : Collect < FuturesOrdered < F > , Vec < F :: Output > > , } , }
};
}
