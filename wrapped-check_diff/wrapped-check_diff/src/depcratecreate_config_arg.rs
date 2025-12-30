// Generated macro for create_config_arg (function)
macro_rules! Depcratecreate_config_arg {
() => {
// Module: crate
// Provides: {"create_config_arg"}
// Dependencies: {}
# [doc = " Creates a configuration in the following form:"] # [doc = " <config_name>=<config_val>, <config_name>=<config_val>, ..."] fn create_config_arg (config : & Option < Vec < String > >) -> String { let config_arg : String = match config { Some (configs) => { let mut result = String :: new () ; for arg in configs . iter () { result . push (',') ; result . push_str (arg . as_str ()) ; } result } None => String :: new () , } ; let config = format ! ("--config=error_on_line_overflow=false,error_on_unformatted=false{}" , config_arg . as_str ()) ; config }
};
}
