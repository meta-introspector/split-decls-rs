// Generated macro for getall_all (function)
macro_rules! Depcrate_stdimplgetall_all {
() => {
// Module: crate::stdimpl
// Provides: {"getall_all"}
// Dependencies: {}
fn getall_all (ctx : Context , cr : & mut Crossroads) -> Option < Context > { get_all_for_path (& ctx . path () . clone () , cr , Some (ctx) , move | ictx , octx | { let props : HashMap < _ , _ > = ictx . ifaces . values () . flatten () . collect () ; octx . as_mut () . unwrap () . do_reply (| msg | { msg . append_all ((props ,)) ; }) ; }) }
};
}
