// Generated macro for is_cloned_or_copied (function)
macro_rules! Depcrate_methods_unnecessary_to_ownedis_cloned_or_copied {
() => {
// Module: crate::methods::unnecessary_to_owned
// Provides: {"is_cloned_or_copied"}
// Dependencies: {}
# [doc = " Returns true if the named method is `Iterator::cloned` or `Iterator::copied`."] fn is_cloned_or_copied (cx : & LateContext < '_ > , method_name : Symbol , method_parent_id : DefId) -> bool { matches ! (method_name , sym :: cloned | sym :: copied) && method_parent_id . is_diag_item (cx , sym :: Iterator) }
};
}
