// Generated macro for derive_debug (function)
macro_rules! Depcrate_mockable_structderive_debug {
() => {
// Module: crate::mockable_struct
// Provides: {"derive_debug"}
// Dependencies: {}
# [doc = " Generate a #[derive(Debug)] Attribute"] fn derive_debug () -> Attribute { let ml = parse2 (quote ! (derive (Debug))) . unwrap () ; Attribute { pound_token : < Token ! [#] > :: default () , style : AttrStyle :: Outer , bracket_token : token :: Bracket :: default () , meta : Meta :: List (ml) } }
};
}
