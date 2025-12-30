// Generated macro for to_adapted_string (function)
macro_rules! Depcrate__macrosto_adapted_string {
() => {
// Module: crate::_macros
// Provides: {"to_adapted_string"}
// Dependencies: {}
# [cfg (feature = "auto")] pub fn to_adapted_string (display : & dyn std :: fmt :: Display , stream : & impl crate :: stream :: RawStream ,) -> String { use std :: io :: Write as _ ; let choice = crate :: AutoStream :: choice (stream) ; let buffer = Vec :: new () ; let mut stream = crate :: AutoStream :: new (buffer , choice) ; let _ = :: std :: write ! (& mut stream , "{display}") ; let buffer = stream . into_inner () ; let buffer = String :: from_utf8_lossy (& buffer) . into_owned () ; buffer }
};
}
