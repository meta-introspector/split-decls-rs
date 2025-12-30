// Generated macro for impl_54 (impl)
macro_rules! Depcrate_corpus_runimpl_54 {
() => {
// Module: crate::corpus::run
// Provides: {"impl_54"}
// Dependencies: {}
impl Execute for OpenRepo { fn execute (& self , repo : & Path , _progress : & mut corpus :: engine :: ProgressItem , _threads : Option < usize > , _should_interrupt : & AtomicBool ,) -> anyhow :: Result < () > { gix :: open_opts (repo , gix :: open :: Options :: isolated ()) ? ; Ok (()) } }
};
}
