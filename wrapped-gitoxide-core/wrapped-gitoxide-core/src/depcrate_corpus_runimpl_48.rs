// Generated macro for impl_48 (impl)
macro_rules! Depcrate_corpus_runimpl_48 {
() => {
// Module: crate::corpus::run
// Provides: {"impl_48"}
// Dependencies: {}
impl Task { pub fn perform (& self , run : & mut Run , repo : & Path , progress : & mut corpus :: engine :: ProgressItem , threads : Option < usize > , should_interrupt : & AtomicBool ,) { let start = std :: time :: Instant :: now () ; if let Err (err) = self . execute . execute (repo , progress , threads , should_interrupt) { run . error = Some (format ! ("{err:#?}")) ; } run . duration = start . elapsed () ; } }
};
}
