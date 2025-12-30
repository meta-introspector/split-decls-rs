// Generated macro for invalid_utf8_contains (function)
macro_rules! Depcrate_stringinvalid_utf8_contains {
() => {
// Module: crate::string
// Provides: {"invalid_utf8_contains"}
// Dependencies: {}
# [doc = " Read the contents of a file that cannot simply be read by"] # [doc = " [`read_to_string`][crate::fs::read_to_string], due to invalid UTF-8 data, then assert"] # [doc = " that it contains `expected`."] # [track_caller] pub fn invalid_utf8_contains < P : AsRef < Path > , S : AsRef < str > > (path : P , expected : S) { let buffer = fs :: read (path . as_ref ()) ; let expected = expected . as_ref () ; if ! String :: from_utf8_lossy (& buffer) . contains (expected) { eprintln ! ("=== FILE CONTENTS (LOSSY) ===") ; eprintln ! ("{}" , String :: from_utf8_lossy (& buffer)) ; eprintln ! ("=== SPECIFIED TEXT ===") ; eprintln ! ("{}" , expected) ; panic ! ("specified text was not found in file") ; } }
};
}
