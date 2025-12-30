// Generated macro for impl_203 (impl)
macro_rules! Depcrate_diagnostics_find_useimpl_203 {
() => {
// Module: crate::diagnostics::find_use
// Provides: {"impl_203"}
// Dependencies: {}
impl < 'a , 'tcx > UseFinder < 'a , 'tcx > { fn find (& mut self) -> Option < Cause > { let mut queue = VecDeque :: new () ; let mut visited = FxIndexSet :: default () ; queue . push_back (self . start_point) ; while let Some (p) = queue . pop_front () { if ! self . regioncx . region_contains (self . region_vid , p) { continue ; } if ! visited . insert (p) { continue ; } let block_data = & self . body [p . block] ; let mut visitor = DefUseVisitor { body : self . body , tcx : self . tcx , region_vid : self . region_vid , def_use_result : None , } ; let is_statement = p . statement_index < block_data . statements . len () ; if is_statement { visitor . visit_statement (& block_data . statements [p . statement_index] , p) ; } else { visitor . visit_terminator (block_data . terminator . as_ref () . unwrap () , p) ; } match visitor . def_use_result { Some (DefUseResult :: Def) => { } Some (DefUseResult :: UseLive { local }) => { return Some (Cause :: LiveVar (local , p)) ; } Some (DefUseResult :: UseDrop { local }) => { return Some (Cause :: DropVar (local , p)) ; } None => { if is_statement { queue . push_back (p . successor_within_block ()) ; } else { queue . extend (block_data . terminator () . successors () . filter (| & bb | { Some (& mir :: UnwindAction :: Cleanup (bb)) != block_data . terminator () . unwind () }) . map (| bb | Location { statement_index : 0 , block : bb }) ,) ; } } } } None } }
};
}
