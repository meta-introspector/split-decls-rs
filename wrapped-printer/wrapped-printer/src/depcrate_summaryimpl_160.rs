// Generated macro for impl_160 (impl)
macro_rules! Depcrate_summaryimpl_160 {
() => {
// Module: crate::summary
// Provides: {"impl_160"}
// Dependencies: {}
impl SummaryKind { # [doc = " Returns true if and only if this output mode requires a file path."] # [doc = ""] # [doc = " When an output mode requires a file path, then the summary printer"] # [doc = " will report an error at the start of every search that lacks a file"] # [doc = " path."] fn requires_path (& self) -> bool { use self :: SummaryKind :: * ; match * self { PathWithMatch | PathWithoutMatch => true , Count | CountMatches | QuietWithMatch | QuietWithoutMatch => false , } } # [doc = " Returns true if and only if this output mode requires computing"] # [doc = " statistics, regardless of whether they have been enabled or not."] fn requires_stats (& self) -> bool { use self :: SummaryKind :: * ; match * self { CountMatches => true , Count | PathWithMatch | PathWithoutMatch | QuietWithMatch | QuietWithoutMatch => false , } } # [doc = " Returns true if and only if a printer using this output mode can"] # [doc = " quit after seeing the first match."] fn quit_early (& self) -> bool { use self :: SummaryKind :: * ; match * self { PathWithMatch | QuietWithMatch => true , Count | CountMatches | PathWithoutMatch | QuietWithoutMatch => { false } } } }
};
}
