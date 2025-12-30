// Generated macro for log_impl (function)
macro_rules! Depcrate___private_apilog_impl {
() => {
// Module: crate::__private_api
// Provides: {"log_impl"}
// Dependencies: {}
fn log_impl < L : Log > (logger : L , args : Arguments , level : Level , & (target , module_path , loc) : & (& str , & 'static str , & 'static Location) , kvs : Option < & [(& str , Value)] > ,) { # [cfg (not (feature = "kv"))] if kvs . is_some () { panic ! ("key-value support is experimental and must be enabled using the `kv` feature") } let mut builder = Record :: builder () ; builder . args (args) . level (level) . target (target) . module_path_static (Some (module_path)) . file_static (Some (loc . file ())) . line (Some (loc . line ())) ; # [cfg (feature = "kv")] builder . key_values (& kvs) ; logger . log (& builder . build ()) ; }
};
}
