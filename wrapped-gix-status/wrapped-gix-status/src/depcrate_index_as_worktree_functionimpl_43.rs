// Generated macro for impl_43 (impl)
macro_rules! Depcrate_index_as_worktree_functionimpl_43 {
() => {
// Module: crate::index_as_worktree::function
// Provides: {"impl_43"}
// Dependencies: {}
impl < 'index , T , U , C : VisitEntry < 'index , ContentChange = T , SubmoduleStatus = U > > Reduce for ReduceChange < '_ , 'index , C > { type Input = Vec < StatusResult < 'index , T , U > > ; type FeedProduce = () ; type Output = () ; type Error = Error ; fn feed (& mut self , items : Self :: Input) -> Result < Self :: FeedProduce , Self :: Error > { for item in items { let (entry , entry_index , path , status) = item ? ; self . collector . visit_entry (self . entries , entry , entry_index , path , status) ; } Ok (()) } fn finalize (self) -> Result < Self :: Output , Self :: Error > { Ok (()) } }
};
}
