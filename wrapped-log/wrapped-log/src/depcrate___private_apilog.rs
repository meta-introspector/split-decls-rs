// Generated macro for log (function)
macro_rules! Depcrate___private_apilog {
() => {
// Module: crate::__private_api
// Provides: {"log"}
// Dependencies: {}
pub fn log < 'a , K , L > (logger : L , args : Arguments , level : Level , target_module_path_and_loc : & (& str , & 'static str , & 'static Location) , kvs : K ,) where K : KVs < 'a > , L : Log , { log_impl (logger , args , level , target_module_path_and_loc , kvs . into_kvs () ,) }
};
}
