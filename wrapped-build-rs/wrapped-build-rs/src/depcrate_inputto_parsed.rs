// Generated macro for to_parsed (function)
macro_rules! Depcrate_inputto_parsed {
() => {
// Module: crate::input
// Provides: {"to_parsed"}
// Dependencies: {}
# [track_caller] fn to_parsed < T > (value : std :: ffi :: OsString) -> T where T : std :: str :: FromStr , T :: Err : std :: fmt :: Display , { let value = to_string (value) ; match value . parse () { Ok (s) => s , Err (err) => { panic ! ("{err}") } } }
};
}
