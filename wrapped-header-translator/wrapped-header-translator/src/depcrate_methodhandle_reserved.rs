// Generated macro for handle_reserved (function)
macro_rules! Depcrate_methodhandle_reserved {
() => {
// Module: crate::method
// Provides: {"handle_reserved"}
// Dependencies: {}
pub (crate) fn handle_reserved (name : & str) -> String { if let Ok (ident) = syn :: parse_str :: < syn :: Ident > (name) { ident . to_string () } else if let Ok (ident) = syn :: parse_str :: < syn :: Ident > (& format ! ("r#{name}")) { ident . to_string () } else if name == "self" { "self_" . into () } else if name == "Self" { "r#Self" . into () } else if name == "super" { "super_" . into () } else if name == "_" { "param1" . into () } else { name . into () } }
};
}
