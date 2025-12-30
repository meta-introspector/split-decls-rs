// Generated macro for send_alert (function)
macro_rules! Depcrate_tlssend_alert {
() => {
// Module: crate::tls
// Provides: {"send_alert"}
// Dependencies: {}
extern "C" fn send_alert (ssl : * mut SSL , level : crypto :: Level , alert : u8 ,) -> c_int { let ex_data = match ExData :: from_ssl_ptr (ssl) { Some (v) => v , None => return 0 , } ; trace ! ("{} send alert lvl={:?} alert={:x}" , ex_data . trace_id , level , alert) ; let error : u64 = TLS_ALERT_ERROR + u64 :: from (alert) ; * ex_data . local_error = Some (ConnectionError { is_app : false , error_code : error , reason : Vec :: new () , }) ; 1 }
};
}
