// Generated macro for async_util (module)
macro_rules! Depcrate_plumbing_mainasync_util {
() => {
// Module: crate::plumbing::main
// Provides: {"async_util"}
// Dependencies: {}
# [cfg (feature = "gitoxide-core-async-client")] pub mod async_util { use crate :: shared :: ProgressRange ; # [cfg (not (feature = "prodash-render-line"))] compile_error ! ("BUG: Need at least a line renderer in async mode") ; pub fn prepare (verbose : bool , trace : bool , name : & str , range : impl Into < Option < ProgressRange > > ,) -> (Option < prodash :: render :: line :: JoinHandle > , gix_features :: progress :: DoOrDiscard < prodash :: tree :: Item > ,) { use crate :: shared :: { self , STANDARD_RANGE } ; shared :: init_env_logger () ; if verbose { let progress = shared :: progress_tree (trace) ; let sub_progress = progress . add_child (name) ; let ui_handle = shared :: setup_line_renderer_range (& progress , range . into () . unwrap_or (STANDARD_RANGE)) ; (Some (ui_handle) , Some (sub_progress) . into ()) } else { (None , None . into ()) } } }
};
}
