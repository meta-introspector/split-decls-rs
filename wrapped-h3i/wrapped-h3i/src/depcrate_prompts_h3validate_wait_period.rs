// Generated macro for validate_wait_period (function)
macro_rules! Depcrate_prompts_h3validate_wait_period {
() => {
// Module: crate::prompts::h3
// Provides: {"validate_wait_period"}
// Dependencies: {}
fn validate_wait_period (period : & str) -> SuggestionResult < Validation > { let x = period . parse :: < u64 > () ; match x { Ok (v) => { let local_conn_timeout = CONNECTION_IDLE_TIMEOUT . with (| v | * v . borrow ()) ; if v >= local_conn_timeout { return Ok (Validation :: Invalid (ErrorMessage :: Custom (format ! ("wait time >= local connection idle timeout {local_conn_timeout}")))) ; } } , Err (_) => return Ok (Validation :: Invalid (ErrorMessage :: Default)) , } Ok (Validation :: Valid) }
};
}
