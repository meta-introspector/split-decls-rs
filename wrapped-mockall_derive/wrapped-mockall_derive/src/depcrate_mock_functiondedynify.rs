// Generated macro for dedynify (function)
macro_rules! Depcrate_mock_functiondedynify {
() => {
// Module: crate::mock_function
// Provides: {"dedynify"}
// Dependencies: {}
# [doc = " Convert a trait object reference into a reference to a Boxed trait"] # [doc = ""] # [doc = " # Returns"] # [doc = ""] # [doc = " Returns `true` if it was necessary to box the type."] fn dedynify (ty : & mut Type) -> bool { if let Type :: Reference (ref mut tr) = ty { if let Type :: TraitObject (ref tto) = tr . elem . as_ref () { if let Some (lt) = & tr . lifetime { if lt . ident == "static" { * tr . elem = parse2 (quote ! ((# tto))) . unwrap () ; return false ; } } * tr . elem = parse2 (quote ! (Box <# tto >)) . unwrap () ; return true ; } } false }
};
}
