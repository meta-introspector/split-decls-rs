// Generated macro for block_def_map (function)
macro_rules! Depcrate_nameresblock_def_map {
() => {
// Module: crate::nameres
// Provides: {"block_def_map"}
// Dependencies: {}
# [salsa_macros :: tracked (returns (ref))] pub fn block_def_map (db : & dyn DefDatabase , block_id : BlockId) -> DefMap { let BlockLoc { ast_id , module } = block_id . lookup (db) ; let visibility = Visibility :: Module (ModuleId { krate : module . krate , local_id : DefMap :: ROOT , block : module . block } , VisibilityExplicitness :: Implicit ,) ; let module_data = ModuleData :: new (ModuleOrigin :: BlockExpr { block : ast_id , id : block_id } , visibility) ; let local_def_map = crate_local_def_map (db , module . krate) ; let def_map = DefMap :: empty (module . krate , local_def_map . def_map (db) . data . clone () , module_data , Some (BlockInfo { block : block_id , parent : BlockRelativeModuleId { block : module . block , local_id : module . local_id } , }) ,) ; let (def_map , _) = collector :: collect_defs (db , def_map , TreeId :: new (ast_id . file_id , Some (block_id)) , Some (local_def_map . local (db)) ,) ; def_map }
};
}
