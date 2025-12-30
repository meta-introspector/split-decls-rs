// Generated macro for prompt_extension (function)
macro_rules! Depcrate_prompts_h3prompt_extension {
() => {
// Module: crate::prompts::h3
// Provides: {"prompt_extension"}
// Dependencies: {}
fn prompt_extension () -> InquireResult < Action > { let stream_id = h3 :: prompt_control_stream_id () ? ; let raw_type = h3 :: prompt_varint ("frame type:") ? ; let payload = Text :: new ("payload:") . with_help_message (ESC_TO_RET) . prompt () . expect ("An error happened when asking for payload, try again later.") ; let fin_stream = prompt_fin_stream () ? ; let action = Action :: SendFrame { stream_id , fin_stream , frame : quiche :: h3 :: frame :: Frame :: Unknown { raw_type , payload : payload . into () , } , } ; Ok (action) }
};
}
