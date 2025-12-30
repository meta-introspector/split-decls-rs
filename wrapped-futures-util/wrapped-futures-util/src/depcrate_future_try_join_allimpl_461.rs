// Generated macro for impl_461 (impl)
macro_rules! Depcrate_future_try_join_allimpl_461 {
() => {
// Module: crate::future::try_join_all
// Provides: {"impl_461"}
// Dependencies: {}
impl < F > fmt :: Debug for TryJoinAll < F > where F : TryFuture + fmt :: Debug , F :: Ok : fmt :: Debug , F :: Error : fmt :: Debug , F :: Output : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . kind { TryJoinAllKind :: Small { ref elems } => { f . debug_struct ("TryJoinAll") . field ("elems" , elems) . finish () } # [cfg_attr (target_os = "none" , cfg (target_has_atomic = "ptr"))] TryJoinAllKind :: Big { ref fut , .. } => fmt :: Debug :: fmt (fut , f) , } } }
};
}
