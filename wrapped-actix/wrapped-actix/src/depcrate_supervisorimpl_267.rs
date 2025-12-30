// Generated macro for impl_267 (impl)
macro_rules! Depcrate_supervisorimpl_267 {
() => {
// Module: crate::supervisor
// Provides: {"impl_267"}
// Dependencies: {}
# [doc (hidden)] impl < A > Future for Supervisor < A > where A : Supervised + Actor < Context = Context < A > > , { type Output = () ; fn poll (self : Pin < & mut Self > , cx : & mut task :: Context < '_ >) -> Poll < Self :: Output > { let mut this = self . project () ; loop { match this . fut . as_mut () . poll (cx) { Poll :: Pending => return Poll :: Pending , Poll :: Ready (_) => { if ! this . fut . restart () { return Poll :: Ready (()) ; } } } } } }
};
}
