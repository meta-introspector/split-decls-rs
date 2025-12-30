// Generated macro for get_data (function)
macro_rules! Depcrate_contextget_data {
() => {
// Module: crate::context
// Provides: {"get_data"}
// Dependencies: {}
fn get_data < 'a > (d : Option < & 'a Json > , p : & str) -> Result < Option < & 'a Json > , RenderError > { let result = match d { Some (Json :: Array (l)) => p . parse :: < usize > () . map (| idx_u | l . get (idx_u)) . map_err (| _ | RenderErrorReason :: InvalidJsonIndex (p . to_owned ())) ? , Some (Json :: Object (m)) => m . get (p) , Some (_) => None , None => None , } ; Ok (result) }
};
}
