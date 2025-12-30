// Generated macro for prompt_transport_or_app (function)
macro_rules! Depcrate_prompts_h3_errorsprompt_transport_or_app {
() => {
// Module: crate::prompts::h3::errors
// Provides: {"prompt_transport_or_app"}
// Dependencies: {}
fn prompt_transport_or_app () -> InquireResult < String > { Ok (Select :: new ("transport or application:" , vec ! [TRANSPORT , APPLICATION]) . prompt () ? . to_string () ,) }
};
}
