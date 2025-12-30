// Generated macro for FindPathCtx (struct)
macro_rules! Depcrate_find_pathFindPathCtx {
() => {
// Module: crate::find_path
// Provides: {"FindPathCtx"}
// Dependencies: {}
struct FindPathCtx < 'db > { db : & 'db dyn DefDatabase , prefix : PrefixKind , cfg : FindPathConfig , ignore_local_imports : bool , is_std_item : bool , from : ModuleId , from_def_map : & 'db DefMap , fuel : Cell < usize > , }
};
}
