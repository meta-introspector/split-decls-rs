// Generated macro for find_crates (function)
macro_rules! Depcrate_pathsfind_crates {
() => {
// Module: crate::paths
// Provides: {"find_crates"}
// Dependencies: {}
# [doc = " Finds the crates called `name`, may be multiple due to multiple major versions."] pub fn find_crates (tcx : TyCtxt < '_ > , name : Symbol) -> & 'static [DefId] { static BY_NAME : OnceLock < FxHashMap < Symbol , Vec < DefId > > > = OnceLock :: new () ; let map = BY_NAME . get_or_init (| | { let mut map = FxHashMap :: default () ; map . insert (tcx . crate_name (LOCAL_CRATE) , vec ! [LOCAL_CRATE . as_def_id ()]) ; for & num in tcx . crates (()) { map . entry (tcx . crate_name (num)) . or_default () . push (num . as_def_id ()) ; } map }) ; match map . get (& name) { Some (def_ids) => def_ids , None => & [] , } }
};
}
