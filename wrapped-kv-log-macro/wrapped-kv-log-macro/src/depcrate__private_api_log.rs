// Generated macro for __private_api_log (function)
macro_rules! Depcrate__private_api_log {
() => {
// Module: crate
// Provides: {"__private_api_log"}
// Dependencies: {}
# [doc (hidden)] pub fn __private_api_log (args : fmt :: Arguments < '_ > , level : Level , & (target , module_path , file , line) : & (& str , & 'static str , & 'static str , u32) , kvs : Option < & [(& str , & dyn log :: kv :: ToValue)] > ,) { logger () . log (& Record :: builder () . args (args) . level (level) . target (target) . module_path_static (Some (module_path)) . file_static (Some (file)) . line (Some (line)) . key_values (& kvs) . build () ,) ; }
};
}
