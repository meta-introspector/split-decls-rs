// Generated macro for run (function)
macro_rules! Depcraterun {
() => {
// Module: crate
// Provides: {"run"}
// Dependencies: {}
# [wasm_bindgen] pub async fn run (repo : String) -> Result < JsValue , JsValue > { let opts = RequestInit :: new () ; opts . set_method ("GET") ; opts . set_mode (RequestMode :: Cors) ; let url = format ! ("https://api.github.com/repos/{repo}/branches/master") ; let request = Request :: new_with_str_and_init (& url , & opts) ? ; request . headers () . set ("Accept" , "application/vnd.github.v3+json") ? ; let window = web_sys :: window () . unwrap () ; let resp_value = JsFuture :: from (window . fetch_with_request (& request)) . await ? ; assert ! (resp_value . is_instance_of ::< Response > ()) ; let resp : Response = resp_value . dyn_into () . unwrap () ; let json = JsFuture :: from (resp . json () ?) . await ? ; Ok (json) }
};
}
