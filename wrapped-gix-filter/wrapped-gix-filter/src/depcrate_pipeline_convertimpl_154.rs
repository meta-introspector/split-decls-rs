// Generated macro for impl_154 (impl)
macro_rules! Depcrate_pipeline_convertimpl_154 {
() => {
// Module: crate::pipeline::convert
// Provides: {"impl_154"}
// Dependencies: {}
impl std :: io :: Read for ToWorktreeOutcome < '_ , '_ > { fn read (& mut self , buf : & mut [u8]) -> std :: io :: Result < usize > { match self { ToWorktreeOutcome :: Unchanged (b) => b . read (buf) , ToWorktreeOutcome :: Buffer (b) => b . read (buf) , ToWorktreeOutcome :: Process (driver :: apply :: MaybeDelayed :: Delayed (_)) => { panic ! ("BUG: must not try to read delayed output") } ToWorktreeOutcome :: Process (driver :: apply :: MaybeDelayed :: Immediate (r)) => r . read (buf) , } } }
};
}
