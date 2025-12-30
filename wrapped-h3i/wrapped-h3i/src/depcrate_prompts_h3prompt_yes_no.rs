// Generated macro for prompt_yes_no (function)
macro_rules! Depcrate_prompts_h3prompt_yes_no {
() => {
// Module: crate::prompts::h3
// Provides: {"prompt_yes_no"}
// Dependencies: {}
fn prompt_yes_no (msg : & str) -> InquireResult < bool > { let res = Select :: new (msg , vec ! [NO , YES]) . prompt () ? ; Ok (res == YES) }
};
}
