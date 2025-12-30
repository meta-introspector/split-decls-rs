// Generated macro for impl_729 (impl)
macro_rules! Depcrate_noteimpl_729 {
() => {
// Module: crate::note
// Provides: {"impl_729"}
// Dependencies: {}
impl < 'repo > Iterator for Notes < 'repo > { type Item = Result < (Oid , Oid) , Error > ; fn next (& mut self) -> Option < Result < (Oid , Oid) , Error > > { let mut note_id = raw :: git_oid { id : [0 ; raw :: GIT_OID_RAWSZ] , } ; let mut annotated_id = note_id ; unsafe { try_call_iter ! (raw :: git_note_next (& mut note_id , & mut annotated_id , self . raw)) ; Some (Ok ((Binding :: from_raw (& note_id as * const _) , Binding :: from_raw (& annotated_id as * const _) ,))) } } }
};
}
