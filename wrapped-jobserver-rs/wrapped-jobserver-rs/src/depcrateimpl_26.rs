// Generated macro for impl_26 (impl)
macro_rules! Depcrateimpl_26 {
() => {
// Module: crate
// Provides: {"impl_26"}
// Dependencies: {}
impl FromEnv { fn new_ok (client : Client , var_name : & 'static str , var_value : OsString) -> FromEnv { FromEnv { client : Ok (client) , var : Some ((var_name , var_value)) , } } fn new_err (kind : FromEnvErrorInner , var_name : & 'static str , var_value : OsString) -> FromEnv { FromEnv { client : Err (FromEnvError { inner : kind }) , var : Some ((var_name , var_value)) , } } }
};
}
