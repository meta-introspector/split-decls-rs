// Generated macro for impl_225 (impl)
macro_rules! Depcrate_rt_synchronizeimpl_225 {
() => {
// Module: crate::rt::synchronize
// Provides: {"impl_225"}
// Dependencies: {}
impl Synchronize { pub fn new () -> Self { Synchronize { happens_before : VersionVec :: new () , } } pub fn sync_load (& mut self , threads : & mut thread :: Set , order : Ordering) { match order { Relaxed | Release => { } Acquire | AcqRel => { self . sync_acq (threads) ; } SeqCst => { self . sync_acq (threads) ; threads . seq_cst () ; } order => unimplemented ! ("unimplemented ordering {:?}" , order) , } } pub fn sync_store (& mut self , threads : & mut thread :: Set , order : Ordering) { self . happens_before . join (& threads . active () . released) ; match order { Relaxed | Acquire => { } Release | AcqRel => { self . sync_rel (threads) ; } SeqCst => { self . sync_rel (threads) ; threads . seq_cst () ; } order => unimplemented ! ("unimplemented ordering {:?}" , order) , } } fn sync_acq (& mut self , threads : & mut thread :: Set) { threads . active_mut () . causality . join (& self . happens_before) ; } fn sync_rel (& mut self , threads : & thread :: Set) { self . happens_before . join (& threads . active () . causality) ; } }
};
}
