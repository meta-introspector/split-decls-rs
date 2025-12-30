// Generated macro for impl_56 (impl)
macro_rules! Depcrate_corpus_runimpl_56 {
() => {
// Module: crate::corpus::run
// Provides: {"impl_56"}
// Dependencies: {}
impl Execute for CountPackedObjects { fn execute (& self , repo : & Path , _progress : & mut corpus :: engine :: ProgressItem , _threads : Option < usize > , _should_interrupt : & AtomicBool ,) -> anyhow :: Result < () > { let repo = gix :: open_opts (repo , gix :: open :: Options :: isolated ()) ? ; repo . objects . packed_object_count () ? ; Ok (()) } }
};
}
