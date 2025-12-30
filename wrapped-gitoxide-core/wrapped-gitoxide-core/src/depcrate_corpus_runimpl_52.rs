// Generated macro for impl_52 (impl)
macro_rules! Depcrate_corpus_runimpl_52 {
() => {
// Module: crate::corpus::run
// Provides: {"impl_52"}
// Dependencies: {}
# [cfg (feature = "archive")] impl Execute for WorktreeStream { fn execute (& self , repo : & Path , progress : & mut corpus :: engine :: ProgressItem , _threads : Option < usize > , should_interrupt : & AtomicBool ,) -> anyhow :: Result < () > { use gix :: Progress ; let repo = gix :: open_opts (repo , gix :: open :: Options :: isolated ()) ? ; let (stream , _) = { let _span = gix :: trace :: coarse ! ("read index and create worktree stream") ; repo . worktree_stream (repo . head_commit () ? . tree_id () ?) ? } ; progress . init (None , gix :: progress :: bytes ()) ; std :: io :: copy (& mut stream . into_read () , & mut gix :: features :: interrupt :: Write { inner : gix :: features :: progress :: Write { inner : std :: io :: sink () , progress , } , should_interrupt , } ,) ? ; Ok (()) } }
};
}
