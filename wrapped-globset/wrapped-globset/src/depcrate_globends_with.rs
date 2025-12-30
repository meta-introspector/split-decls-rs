// Generated macro for ends_with (function)
macro_rules! Depcrate_globends_with {
() => {
// Module: crate::glob
// Provides: {"ends_with"}
// Dependencies: {}
# [cfg (test)] fn ends_with (needle : & [u8] , haystack : & [u8]) -> bool { if needle . len () > haystack . len () { return false ; } needle == & haystack [haystack . len () - needle . len () ..] }
};
}
