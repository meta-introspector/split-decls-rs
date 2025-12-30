// Generated macro for impl_58 (impl)
macro_rules! Depcrate_corpus_runimpl_58 {
() => {
// Module: crate::corpus::run
// Provides: {"impl_58"}
// Dependencies: {}
impl Execute for VerifyOdb { fn execute (& self , repo : & Path , progress : & mut corpus :: engine :: ProgressItem , threads : Option < usize > , should_interrupt : & AtomicBool ,) -> anyhow :: Result < () > { let repo = gix :: open_opts (repo , gix :: open :: Options :: isolated ()) ? ; crate :: repository :: verify :: integrity (repo , std :: io :: sink () , progress . add_child ("integrity" . into ()) , should_interrupt , crate :: repository :: verify :: Context { output_statistics : None , thread_limit : threads , verify_mode : Default :: default () , algorithm : Algorithm :: LessTime , } ,) ? ; Ok (()) } }
};
}
