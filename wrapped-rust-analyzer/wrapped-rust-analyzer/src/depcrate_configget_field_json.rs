// Generated macro for get_field_json (function)
macro_rules! Depcrate_configget_field_json {
() => {
// Module: crate::config
// Provides: {"get_field_json"}
// Dependencies: {}
fn get_field_json < T : DeserializeOwned > (json : & mut serde_json :: Value , error_sink : & mut Vec < (String , serde_json :: Error) > , field : & 'static str , alias : Option < & 'static str > ,) -> Option < T > { alias . into_iter () . chain (iter :: once (field)) . filter_map (move | field | { let mut pointer = field . replace ('_' , "/") ; pointer . insert (0 , '/') ; json . pointer_mut (& pointer) . map (| it | serde_json :: from_value (it . take ()) . map_err (| e | (e , pointer))) }) . flat_map (| res | match res { Ok (it) => Some (it) , Err ((e , pointer)) => { tracing :: warn ! ("Failed to deserialize config field at {}: {:?}" , pointer , e) ; error_sink . push ((pointer , e)) ; None } }) . next () }
};
}
