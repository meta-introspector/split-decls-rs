// Generated macro for CapturedState (struct)
macro_rules! Depcrate_continuationCapturedState {
() => {
// Module: crate::continuation
// Provides: {"CapturedState"}
// Dependencies: {}
# [doc = " Placeholder for the compiler's captured state."] # [derive (Debug , Clone , Serialize , Deserialize)] pub struct CapturedState { pub session_id : String , pub call_context : String , pub stack_trace : Vec < String > , # [serde (flatten)] pub variables : serde_json :: Value , pub file_path : String , pub line_number : u32 , pub column_number : u32 , }
};
}
