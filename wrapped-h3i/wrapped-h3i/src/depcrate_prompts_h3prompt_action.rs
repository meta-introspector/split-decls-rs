// Generated macro for prompt_action (function)
macro_rules! Depcrate_prompts_h3prompt_action {
() => {
// Module: crate::prompts::h3
// Provides: {"prompt_action"}
// Dependencies: {}
fn prompt_action () -> InquireResult < String > { let name = Text :: new ("Select an action to queue. `Commit` ends selection and flushes queue." ,) . with_autocomplete (& action_suggester) . with_page_size (18) . prompt () ; name }
};
}
