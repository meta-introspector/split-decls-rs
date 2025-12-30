// Generated macro for app (function)
macro_rules! Depcrateapp {
() => {
// Module: crate
// Provides: {"app"}
// Dependencies: {}
fn app () -> Router { let state = AppState { db : Arc :: new (RwLock :: new (HashMap :: new ())) , } ; let sensitive_headers : Arc < [_] > = vec ! [header :: AUTHORIZATION , header :: COOKIE] . into () ; let middleware = ServiceBuilder :: new () . sensitive_request_headers (sensitive_headers . clone ()) . layer (TraceLayer :: new_for_http () . on_body_chunk (| chunk : & Bytes , latency : Duration , _ : & tracing :: Span | { tracing :: trace ! (size_bytes = chunk . len () , latency = ? latency , "sending body chunk") }) . make_span_with (DefaultMakeSpan :: new () . include_headers (true)) . on_response (DefaultOnResponse :: new () . include_headers (true) . latency_unit (LatencyUnit :: Micros)) ,) . sensitive_response_headers (sensitive_headers) . layer (TimeoutLayer :: with_status_code (StatusCode :: REQUEST_TIMEOUT , Duration :: from_secs (10))) . compression () . insert_response_header_if_not_present (header :: CONTENT_TYPE , HeaderValue :: from_static ("application/octet-stream") ,) ; Router :: new () . route ("/{key}" , get (get_key) . post (set_key)) . layer (middleware) . with_state (state) }
};
}
