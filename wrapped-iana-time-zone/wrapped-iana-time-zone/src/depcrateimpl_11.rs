// Generated macro for impl_11 (impl)
macro_rules! Depcrateimpl_11 {
() => {
// Module: crate
// Provides: {"impl_11"}
// Dependencies: {}
impl std :: error :: Error for GetTimezoneError { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match self { GetTimezoneError :: FailedParsingString => None , GetTimezoneError :: IoError (err) => Some (err) , GetTimezoneError :: OsError => None , } } }
};
}
