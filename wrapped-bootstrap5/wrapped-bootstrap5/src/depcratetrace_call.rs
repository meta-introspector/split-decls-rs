// Generated macro for trace_call (macro)
macro_rules! Depcratetrace_call {
() => {
// Module: crate
// Provides: {"trace_call"}
// Dependencies: {}
macro_rules ! trace_call { ($ func_name : expr , $ func_call : expr) => { { let start = Instant :: now () ; unsafe { CALL_COUNTER += 1 ; let current_id = CALL_COUNTER ; let parent_id = if CALL_STACK . is_empty () { None } else { Some (CALL_COUNTER - 1) } ; let depth = CALL_STACK . len () ; CALL_STACK . push ($ func_name . to_string ()) ; println ! ("{}📞 [TRACE] {} (ID: {}, Depth: {})" , "  " . repeat (depth) , $ func_name , current_id , depth) ; let result = $ func_call ; let duration = start . elapsed () ; let success = result . is_ok () ; CALL_TREE . push (CallTrace { call_id : current_id , function_name : $ func_name . to_string () , parent_id , depth , duration_ms : duration . as_millis () , success , }) ; CALL_STACK . pop () ; println ! ("{}✅ [TRACE] {} completed ({}ms)" , "  " . repeat (depth) , $ func_name , duration . as_millis ()) ; result } } } ; }
};
}
