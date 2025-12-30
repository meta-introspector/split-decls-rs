// Generated macro for literal_nocapture (function)
macro_rules! Depcrate_parseliteral_nocapture {
() => {
// Module: crate::parse
// Provides: {"literal_nocapture"}
// Dependencies: {}
fn literal_nocapture (input : Cursor) -> Result < Cursor , Reject > { if let Ok (ok) = string (input) { Ok (ok) } else if let Ok (ok) = byte_string (input) { Ok (ok) } else if let Ok (ok) = c_string (input) { Ok (ok) } else if let Ok (ok) = byte (input) { Ok (ok) } else if let Ok (ok) = character (input) { Ok (ok) } else if let Ok (ok) = float (input) { Ok (ok) } else if let Ok (ok) = int (input) { Ok (ok) } else { Err (Reject) } }
};
}
