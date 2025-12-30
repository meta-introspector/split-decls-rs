// Generated macro for impl_155 (impl)
macro_rules! Depcrate_pipeline_convertimpl_155 {
() => {
// Module: crate::pipeline::convert
// Provides: {"impl_155"}
// Dependencies: {}
impl < R > std :: io :: Read for ToGitOutcome < '_ , R > where R : std :: io :: Read , { fn read (& mut self , buf : & mut [u8]) -> std :: io :: Result < usize > { match self { ToGitOutcome :: Unchanged (r) => r . read (buf) , ToGitOutcome :: Process (r) => r . read (buf) , ToGitOutcome :: Buffer (r) => r . read (buf) , } } }
};
}
