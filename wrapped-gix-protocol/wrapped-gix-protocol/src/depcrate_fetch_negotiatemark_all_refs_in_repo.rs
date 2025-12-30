// Generated macro for mark_all_refs_in_repo (function)
macro_rules! Depcrate_fetch_negotiatemark_all_refs_in_repo {
() => {
// Module: crate::fetch::negotiate
// Provides: {"mark_all_refs_in_repo"}
// Dependencies: {}
fn mark_all_refs_in_repo (store : & gix_ref :: file :: Store , objects : & impl gix_object :: Find , graph : & mut gix_negotiate :: Graph < '_ , '_ > , queue : & mut Queue , mark : Flags ,) -> Result < () , Error > { let _span = gix_trace :: detail ! ("mark_all_refs") ; for local_ref in store . iter () ? . all () ? { let mut local_ref = local_ref ? ; let id = local_ref . peel_to_id_packed (store , objects , store . cached_packed_buffer () ? . as_ref () . map (| b | & * * * b)) ? ; let mut is_complete = false ; if let Some (commit) = graph . get_or_insert_commit (id , | md | { is_complete = md . flags . contains (Flags :: COMPLETE) ; md . flags |= mark ; }) ? . filter (| _ | ! is_complete) { queue . insert (commit . commit_time , id) ; } ; } Ok (()) }
};
}
