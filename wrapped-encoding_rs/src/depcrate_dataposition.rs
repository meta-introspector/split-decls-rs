// Generated macro for position (function)
macro_rules! Depcrate_dataposition {
() => {
// Module: crate::data
// Provides: {"position"}
// Dependencies: {}
# [inline (always)] pub fn position (haystack : & [u16] , needle : u16) -> Option < usize > { haystack . iter () . position (| & x | x == needle) }
};
}
