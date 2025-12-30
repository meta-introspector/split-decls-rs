// Generated macro for get_field_toml (function)
macro_rules! Depcrate_configget_field_toml {
() => {
// Module: crate::config
// Provides: {"get_field_toml"}
// Dependencies: {}
fn get_field_toml < T : DeserializeOwned > (toml : & toml :: Table , error_sink : & mut Vec < (String , toml :: de :: Error) > , field : & 'static str , alias : Option < & 'static str > ,) -> Option < T > { alias . into_iter () . chain (iter :: once (field)) . filter_map (move | field | { let mut pointer = field . replace ('_' , "/") ; pointer . insert (0 , '/') ; toml_pointer (toml , & pointer) . map (| it | < _ > :: deserialize (it . clone ()) . map_err (| e | (e , pointer))) }) . find (Result :: is_ok) . and_then (| res | match res { Ok (it) => Some (it) , Err ((e , pointer)) => { tracing :: warn ! ("Failed to deserialize config field at {}: {:?}" , pointer , e) ; error_sink . push ((pointer , e)) ; None } }) }
};
}
