// Generated macro for HostInfoError (enum)
macro_rules! Depcrate_errorHostInfoError {
() => {
// Module: crate::error
// Provides: {"HostInfoError"}
// Dependencies: {}
# [doc = " An error encountered while retrieving the host information"] # [derive (Debug , Display)] pub enum HostInfoError { # [displaydoc ("Error converting into `&CStr` to `&str`")] Conversion (Utf8Error) , # [displaydoc ("Error creating a `CString` from a buffer with a null terminator")] FromVecWithNul (FromVecWithNulError) , # [displaydoc ("No matching backend has been identified")] UnavailableBackend , # [displaydoc ("Unknown category when retrieving locale category for linux")] UnknownCategory , # [cfg (target_os = "windows")] # [displaydoc ("Windows error: {0}")] Windows (windows :: core :: Error) , # [displaydoc ("Host locale parsing error")] HostLocaleError , # [displaydoc ("Failed to parse region")] UnknownRegion , # [displaydoc ("Failed to parse a locale: {0}")] LocaleParse (ParseError) , }
};
}
