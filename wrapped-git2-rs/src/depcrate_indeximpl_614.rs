// Generated macro for impl_614 (impl)
macro_rules! Depcrate_indeximpl_614 {
() => {
// Module: crate::index
// Provides: {"impl_614"}
// Dependencies: {}
impl < 'index > Iterator for IndexConflicts < 'index > { type Item = Result < IndexConflict , Error > ; fn next (& mut self) -> Option < Result < IndexConflict , Error > > { let mut ancestor = ptr :: null () ; let mut our = ptr :: null () ; let mut their = ptr :: null () ; unsafe { try_call_iter ! (raw :: git_index_conflict_next (& mut ancestor , & mut our , & mut their , self . conflict_iter)) ; Some (Ok (IndexConflict { ancestor : match ancestor . is_null () { false => Some (IndexEntry :: from_raw (* ancestor)) , true => None , } , our : match our . is_null () { false => Some (IndexEntry :: from_raw (* our)) , true => None , } , their : match their . is_null () { false => Some (IndexEntry :: from_raw (* their)) , true => None , } , })) } } }
};
}
