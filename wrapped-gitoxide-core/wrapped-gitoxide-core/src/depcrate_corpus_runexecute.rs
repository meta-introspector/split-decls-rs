// Generated macro for Execute (trait)
macro_rules! Depcrate_corpus_runExecute {
() => {
// Module: crate::corpus::run
// Provides: {"Execute"}
// Dependencies: {}
# [doc = " Note that once runs have been recorded, the implementation must not change anymore to keep it comparable."] # [doc = " If changes have be done, rather change the name of the owning task to start a new kind of task."] pub (crate) trait Execute { fn execute (& self , repo : & Path , progress : & mut corpus :: engine :: ProgressItem , threads : Option < usize > , should_interrupt : & AtomicBool ,) -> anyhow :: Result < () > ; }
};
}
