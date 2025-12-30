// Generated macro for wait_type_validator (function)
macro_rules! Depcrate_prompts_h3_waitwait_type_validator {
() => {
// Module: crate::prompts::h3::wait
// Provides: {"wait_type_validator"}
// Dependencies: {}
fn wait_type_validator (wait_type : & str) -> SuggestionResult < Validation > { match wait_type { DURATION | HEADERS | DATA | FINISHED => Ok (Validation :: Valid) , _ => Ok (Validation :: Invalid (inquire :: validator :: ErrorMessage :: Default ,)) , } }
};
}
