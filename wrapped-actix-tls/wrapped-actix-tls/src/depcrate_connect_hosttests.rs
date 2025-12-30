// Generated macro for tests (module)
macro_rules! Depcrate_connect_hosttests {
() => {
// Module: crate::connect::host
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; macro_rules ! assert_connection_info_eq { ($ req : expr , $ hostname : expr , $ port : expr) => { { assert_eq ! ($ req . hostname () , $ hostname) ; assert_eq ! ($ req . port () , $ port) ; } } ; } # [test] fn host_parsing () { assert_connection_info_eq ! ("example.com" , "example.com" , None) ; assert_connection_info_eq ! ("example.com:8080" , "example.com" , Some (8080)) ; assert_connection_info_eq ! ("example:8080" , "example" , Some (8080)) ; assert_connection_info_eq ! ("example.com:false" , "example.com" , None) ; assert_connection_info_eq ! ("example.com:false:false" , "example.com" , None) ; } }
};
}
