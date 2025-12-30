// Generated macro for impl_2804 (impl)
macro_rules! Depcrate_traversalsimpl_2804 {
() => {
// Module: crate::traversals
// Provides: {"impl_2804"}
// Dependencies: {}
impl Iterator for DfsIter < '_ > { type Item = (Event , ir :: Block) ; fn next (& mut self) -> Option < (Event , ir :: Block) > { let (event , block) = self . dfs . stack . pop () ? ; if event == Event :: Enter && self . dfs . seen . insert (block) { self . dfs . stack . push ((Event :: Exit , block)) ; self . dfs . stack . extend (self . func . block_successors (block) . rev () . filter (| block | ! self . dfs . seen . contains (* block)) . map (| block | (Event :: Enter , block)) ,) ; } Some ((event , block)) } }
};
}
