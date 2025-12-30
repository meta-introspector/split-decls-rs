// Generated macro for prompt_grease (function)
macro_rules! Depcrate_prompts_h3prompt_grease {
() => {
// Module: crate::prompts::h3
// Provides: {"prompt_grease"}
// Dependencies: {}
fn prompt_grease () -> InquireResult < Action > { let stream_id = h3 :: prompt_control_stream_id () ? ; let raw_type = quiche :: h3 :: grease_value () ; let payload = Text :: new ("payload:") . prompt () . expect ("An error happened when asking for payload, try again later.") ; let fin_stream = prompt_fin_stream () ? ; let action = Action :: SendFrame { stream_id , fin_stream , frame : quiche :: h3 :: frame :: Frame :: Unknown { raw_type , payload : payload . into () , } , } ; Ok (action) }
};
}
