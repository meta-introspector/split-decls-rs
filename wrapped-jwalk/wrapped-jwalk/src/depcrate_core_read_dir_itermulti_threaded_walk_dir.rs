// Generated macro for multi_threaded_walk_dir (function)
macro_rules! Depcrate_core_read_dir_itermulti_threaded_walk_dir {
() => {
// Module: crate::core::read_dir_iter
// Provides: {"multi_threaded_walk_dir"}
// Dependencies: {}
fn multi_threaded_walk_dir < C : ClientState > (ordered_read_dir_spec : Ordered < ReadDirSpec < C > > , run_context : & mut RunContext < C > ,) { let Ordered { value : read_dir_spec , index_path , .. } = ordered_read_dir_spec ; let read_dir_result = (run_context . core_read_dir_callback) (read_dir_spec) ; let ordered_read_children_specs = read_dir_result . as_ref () . ok () . map (| read_dir | read_dir . ordered_read_children_specs (& index_path)) ; let ordered_read_dir_result = Ordered :: new (read_dir_result , index_path , ordered_read_children_specs . as_ref () . map_or (0 , Vec :: len) ,) ; if ! run_context . send_read_dir_result (ordered_read_dir_result) { run_context . stop () ; return ; } if let Some (ordered_read_children_specs) = ordered_read_children_specs { for each in ordered_read_children_specs { if ! run_context . schedule_read_dir_spec (each) { run_context . stop () ; return ; } } } run_context . complete_item () ; }
};
}
