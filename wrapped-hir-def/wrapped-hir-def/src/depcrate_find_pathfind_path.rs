// Generated macro for find_path (function)
macro_rules! Depcrate_find_pathfind_path {
() => {
// Module: crate::find_path
// Provides: {"find_path"}
// Dependencies: {}
# [doc = " Find a path that can be used to refer to a certain item. This can depend on"] # [doc = " *from where* you're referring to the item, hence the `from` parameter."] pub fn find_path (db : & dyn DefDatabase , item : ItemInNs , from : ModuleId , mut prefix_kind : PrefixKind , ignore_local_imports : bool , mut cfg : FindPathConfig ,) -> Option < ModPath > { let _p = tracing :: info_span ! ("find_path") . entered () ; if let ItemInNs :: Types (ModuleDefId :: BuiltinType (builtin)) = item { return Some (ModPath :: from_segments (PathKind :: Plain , iter :: once (builtin . as_name ()))) ; } let item_module = item . module (db) ? ; if item_module . is_within_block () { prefix_kind = PrefixKind :: Plain ; } cfg . prefer_no_std = cfg . prefer_no_std || db . crate_supports_no_std (from . krate ()) ; find_path_inner (& FindPathCtx { db , prefix : prefix_kind , cfg , ignore_local_imports , is_std_item : item_module . krate () . data (db) . origin . is_lang () , from , from_def_map : from . def_map (db) , fuel : Cell :: new (FIND_PATH_FUEL) , } , item , MAX_PATH_LEN ,) }
};
}
