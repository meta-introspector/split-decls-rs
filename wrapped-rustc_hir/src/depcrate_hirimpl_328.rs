// Generated macro for impl_328 (impl)
macro_rules! Depcrate_hirimpl_328 {
() => {
// Module: crate::hir
// Provides: {"impl_328"}
// Dependencies: {}
impl fmt :: Display for LoopIdError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (match self { LoopIdError :: OutsideLoopScope => "not inside loop scope" , LoopIdError :: UnlabeledCfInWhileCondition => { "unlabeled control flow (break or continue) in while condition" } LoopIdError :: UnresolvedLabel => "label not found" , }) } }
};
}
