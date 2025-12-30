// Generated macro for ReflogEntry (struct)
macro_rules! Depcrate_reflogReflogEntry {
() => {
// Module: crate::reflog
// Provides: {"ReflogEntry"}
// Dependencies: {}
# [doc = " An entry inside the reflog of a repository"] pub struct ReflogEntry < 'reflog > { raw : * const raw :: git_reflog_entry , _marker : marker :: PhantomData < & 'reflog Reflog > , }
};
}
