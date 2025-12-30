// Generated macro for get_status_code (function)
macro_rules! Depcrateget_status_code {
() => {
// Module: crate
// Provides: {"get_status_code"}
// Dependencies: {}
# [cfg (windows)] fn get_status_code (status : & ExitStatus) -> (u8 , i32) { (0 , status . code () . unwrap ()) }
};
}
