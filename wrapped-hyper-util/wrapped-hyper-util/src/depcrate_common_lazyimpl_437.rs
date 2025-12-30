// Generated macro for impl_437 (impl)
macro_rules! Depcrate_common_lazyimpl_437 {
() => {
// Module: crate::common::lazy
// Provides: {"impl_437"}
// Dependencies: {}
impl < F , R > Future for Lazy < F , R > where F : FnOnce () -> R , R : Future , { type Output = R :: Output ; fn poll (self : Pin < & mut Self > , cx : & mut task :: Context < '_ >) -> Poll < Self :: Output > { let mut this = self . project () ; if let InnerProj :: Fut { fut } = this . inner . as_mut () . project () { return fut . poll (cx) ; } match this . inner . as_mut () . project_replace (Inner :: Empty) { InnerProjReplace :: Init { func } => { this . inner . set (Inner :: Fut { fut : func () }) ; if let InnerProj :: Fut { fut } = this . inner . project () { return fut . poll (cx) ; } unreachable ! () } _ => unreachable ! ("lazy state wrong") , } } }
};
}
