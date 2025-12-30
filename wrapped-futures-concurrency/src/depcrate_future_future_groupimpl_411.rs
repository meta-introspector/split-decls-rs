// Generated macro for impl_411 (impl)
macro_rules! Depcrate_future_future_groupimpl_411 {
() => {
// Module: crate::future::future_group
// Provides: {"impl_411"}
// Dependencies: {}
impl < F : Future > Stream for Keyed < F > { type Item = (Key , < F as Future > :: Output) ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut this = self . project () ; this . group . as_mut () . poll_next_inner (cx) } }
};
}
