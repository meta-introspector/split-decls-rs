// Generated macro for macro_188 (macro)
macro_rules! Depcrate_sync_sharded_lockmacro_188 {
() => {
// Module: crate::sync::sharded_lock
// Provides: {"macro_188"}
// Dependencies: {}
std :: thread_local ! { static REGISTRATION : Registration = { let thread_id = thread :: current () . id () ; let mut indices = thread_indices () . lock () . unwrap () ; let index = match indices . free_list . pop () { Some (i) => i , None => { let i = indices . next_index ; indices . next_index += 1 ; i } } ; indices . mapping . insert (thread_id , index) ; Registration { index , thread_id , } } ; }
};
}
