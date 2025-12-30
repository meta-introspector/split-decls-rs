// Generated macro for as_c_return_code (function)
macro_rules! Depcrateas_c_return_code {
() => {
// Module: crate
// Provides: {"as_c_return_code"}
// Dependencies: {}
fn as_c_return_code (r : MZResult) -> c_int { match r { Err (status) => status as c_int , Ok (status) => status as c_int , } }
};
}
