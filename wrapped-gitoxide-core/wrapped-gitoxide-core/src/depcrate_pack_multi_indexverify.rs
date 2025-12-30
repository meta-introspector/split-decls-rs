// Generated macro for verify (function)
macro_rules! Depcrate_pack_multi_indexverify {
() => {
// Module: crate::pack::multi_index
// Provides: {"verify"}
// Dependencies: {}
pub fn verify (multi_index_path : PathBuf , mut progress : impl NestedProgress + 'static , should_interrupt : & AtomicBool ,) -> anyhow :: Result < () > { gix :: odb :: pack :: multi_index :: File :: at (multi_index_path) ? . verify_integrity_fast (& mut progress , should_interrupt) ? ; Ok (()) }
};
}
