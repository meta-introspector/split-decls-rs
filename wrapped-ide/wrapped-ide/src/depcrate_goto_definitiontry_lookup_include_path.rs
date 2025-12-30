// Generated macro for try_lookup_include_path (function)
macro_rules! Depcrate_goto_definitiontry_lookup_include_path {
() => {
// Module: crate::goto_definition
// Provides: {"try_lookup_include_path"}
// Dependencies: {}
fn try_lookup_include_path (sema : & Semantics < '_ , RootDatabase > , token : InFile < ast :: String > , file_id : FileId ,) -> Option < NavigationTarget > { let file = token . file_id . macro_file () ? ; if ! iter :: successors (Some (file) , | file | file . parent (sema . db) . macro_file ()) . any (| file | file . is_include_like_macro (sema . db) && file . eager_arg (sema . db) . is_none ()) { return None ; } let path = token . value . value () . ok () ? ; let file_id = sema . db . resolve_path (AnchoredPath { anchor : file_id , path : & path }) ? ; let size = sema . db . file_text (file_id) . text (sema . db) . len () . try_into () . ok () ? ; Some (NavigationTarget { file_id , full_range : TextRange :: new (0 . into () , size) , name : hir :: Symbol :: intern (& path) , alias : None , focus_range : None , kind : None , container_name : None , description : None , docs : None , }) }
};
}
