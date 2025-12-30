// Generated macro for first_type_path_segment_ident (function)
macro_rules! Depcrate_utilsfirst_type_path_segment_ident {
() => {
// Module: crate::utils
// Provides: {"first_type_path_segment_ident"}
// Dependencies: {}
fn first_type_path_segment_ident (t : & Type) -> Option < & Ident > { match t { Type :: Path (tp) if tp . qself . is_none () && tp . path . leading_colon . is_none () => tp . path . segments . iter () . next () . and_then (| ps | match ps . arguments { syn :: PathArguments :: None => Some (& ps . ident) , _ => None , }) , _ => None , } }
};
}
