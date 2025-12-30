// Generated macro for crate_local_def_map (function)
macro_rules! Depcrate_namerescrate_local_def_map {
() => {
// Module: crate::nameres
// Provides: {"crate_local_def_map"}
// Dependencies: {}
# [salsa_macros :: tracked (returns (ref))] pub (crate) fn crate_local_def_map (db : & dyn DefDatabase , crate_id : Crate) -> DefMapPair < '_ > { let krate = crate_id . data (db) ; let _p = tracing :: info_span ! ("crate_def_map_query" , name =? crate_id . extra_data (db) . display_name . as_ref () . map (| it | it . crate_name () . to_smolstr ()) . unwrap_or_default ()) . entered () ; let module_data = ModuleData :: new (ModuleOrigin :: CrateRoot { definition : krate . root_file_id (db) } , Visibility :: Public ,) ; let def_map = DefMap :: empty (crate_id , Arc :: new (DefMapCrateData :: new (krate . edition)) , module_data , None) ; let (def_map , local_def_map) = collector :: collect_defs (db , def_map , TreeId :: new (krate . root_file_id (db) . into () , None) , None ,) ; DefMapPair :: new (db , def_map , local_def_map) }
};
}
