// Generated macro for impl_21 (impl)
macro_rules! Depcrateimpl_21 {
() => {
// Module: crate
// Provides: {"impl_21"}
// Dependencies: {}
impl FromStr for QueryKey { type Err = QueryKeyError ; fn from_str (s : & str) -> Result < Self , Self :: Err > { let lowercase = s . to_ascii_lowercase () ; match lowercase . as_str () { QUERY_KEY_FULL_PATH => Ok (Self :: FullPath) , QUERY_KEY_KEY => Ok (Self :: Key) , _ => Err (QueryKeyError (s . to_string ())) , } } }
};
}
