// Generated macro for impl_322 (impl)
macro_rules! Depcrate_hybrid_regeximpl_322 {
() => {
// Module: crate::hybrid::regex
// Provides: {"impl_322"}
// Dependencies: {}
impl < 'r , 'c , 'h > Iterator for FindMatches < 'r , 'c , 'h > { type Item = Match ; # [inline] fn next (& mut self) -> Option < Match > { let FindMatches { re , ref mut cache , ref mut it } = * self ; it . advance (| input | re . try_search (cache , input)) } }
};
}
