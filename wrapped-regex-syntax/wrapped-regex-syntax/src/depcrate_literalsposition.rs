// Generated macro for position (function)
macro_rules! Depcrate_literalsposition {
() => {
// Module: crate::literals
// Provides: {"position"}
// Dependencies: {}
fn position (needle : & [u8] , mut haystack : & [u8]) -> Option < usize > { let mut i = 0 ; while haystack . len () >= needle . len () { if needle == & haystack [.. needle . len ()] { return Some (i) ; } i += 1 ; haystack = & haystack [1 ..] ; } None }
};
}
