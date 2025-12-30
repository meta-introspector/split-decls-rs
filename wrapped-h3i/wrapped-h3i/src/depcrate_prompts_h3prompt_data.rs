// Generated macro for prompt_data (function)
macro_rules! Depcrate_prompts_h3prompt_data {
() => {
// Module: crate::prompts::h3
// Provides: {"prompt_data"}
// Dependencies: {}
fn prompt_data () -> InquireResult < Action > { let stream_id = h3 :: prompt_stream_id () ? ; let payload = Text :: new ("payload:") . prompt () ? ; let fin_stream = prompt_fin_stream () ? ; let action = Action :: SendFrame { stream_id , fin_stream , frame : quiche :: h3 :: frame :: Frame :: Data { payload : payload . into () , } , } ; Ok (action) }
};
}
