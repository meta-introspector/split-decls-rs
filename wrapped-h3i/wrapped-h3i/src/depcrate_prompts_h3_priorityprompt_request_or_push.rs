// Generated macro for prompt_request_or_push (function)
macro_rules! Depcrate_prompts_h3_priorityprompt_request_or_push {
() => {
// Module: crate::prompts::h3::priority
// Provides: {"prompt_request_or_push"}
// Dependencies: {}
fn prompt_request_or_push () -> InquireResult < String > { Ok (Select :: new ("request or push:" , vec ! [REQUEST , PUSH]) . prompt () ? . to_string ()) }
};
}
