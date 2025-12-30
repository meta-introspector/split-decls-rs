// Generated macro for prompt_wait_period (function)
macro_rules! Depcrate_prompts_h3_waitprompt_wait_period {
() => {
// Module: crate::prompts::h3::wait
// Provides: {"prompt_wait_period"}
// Dependencies: {}
pub fn prompt_wait_period () -> InquireResult < WaitType > { let period = Text :: new ("wait period (ms):") . with_validator (validate_wait_period) . prompt () ? ; let period = Duration :: from_millis (period . parse :: < u64 > () . unwrap ()) ; Ok (WaitType :: WaitDuration (period)) }
};
}
