// Generated macro for impl_8 (impl)
macro_rules! Depcrate_apply_changeimpl_8 {
() => {
// Module: crate::apply_change
// Provides: {"impl_8"}
// Dependencies: {}
impl RootDatabase { pub fn request_cancellation (& mut self) { let _p = tracing :: info_span ! ("RootDatabase::request_cancellation") . entered () ; self . synthetic_write (Durability :: LOW) ; } pub fn apply_change (& mut self , change : ChangeWithProcMacros) { let _p = tracing :: info_span ! ("RootDatabase::apply_change") . entered () ; self . request_cancellation () ; tracing :: trace ! ("apply_change {:?}" , change) ; if let Some (roots) = & change . source_change . roots { let mut local_roots = FxHashSet :: default () ; let mut library_roots = FxHashSet :: default () ; for (idx , root) in roots . iter () . enumerate () { let root_id = SourceRootId (idx as u32) ; if root . is_library { library_roots . insert (root_id) ; } else { local_roots . insert (root_id) ; } } LocalRoots :: get (self) . set_roots (self) . to (local_roots) ; LibraryRoots :: get (self) . set_roots (self) . to (library_roots) ; } change . apply (self) ; } pub fn per_query_memory_usage (& mut self) -> Vec < (String , Bytes , usize) > { let mut acc : Vec < (String , Bytes , usize) > = vec ! [] ; macro_rules ! purge_each_query { ($ ($ q : path) *) => { $ (let before = memory_usage () . allocated ; let table = $ q . in_db (self) ; let count = collect_query_count (& table) ; table . purge () ; let after = memory_usage () . allocated ; let q : $ q = Default :: default () ; let name = format ! ("{:?}" , q) ; acc . push ((name , before - after , count)) ;) * } } purge_each_query ! [] ; acc . sort_by_key (| it | std :: cmp :: Reverse (it . 1)) ; acc } }
};
}
