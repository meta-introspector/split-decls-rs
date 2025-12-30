// Generated macro for impl_54 (impl)
macro_rules! Depcrate_http_queryimpl_54 {
() => {
// Module: crate::http::query
// Provides: {"impl_54"}
// Dependencies: {}
# [doc = " The formatted query parameters ready to be used in a URL query string."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " The resulting string does not contain a leading `?` and is properly encoded:"] # [doc = ""] # [doc = " ```"] # [doc = " # fn no_run() {"] # [doc = " use gloo_net::http::QueryParams;"] # [doc = ""] # [doc = " let params = QueryParams::new();"] # [doc = " params.append(\"a\", \"1\");"] # [doc = " params.append(\"b\", \"2\");"] # [doc = " assert_eq!(params.to_string(), \"a=1&b=2\".to_string());"] # [doc = ""] # [doc = " params.append(\"key\", \"ab&c\");"] # [doc = " assert_eq!(params.to_string(), \"a=1&b=2&key=ab%26c\");"] # [doc = " # }"] # [doc = " ```"] impl fmt :: Display for QueryParams { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "{}" , self . raw . to_string ()) } }
};
}
