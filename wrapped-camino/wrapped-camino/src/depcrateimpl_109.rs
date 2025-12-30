// Generated macro for impl_109 (impl)
macro_rules! Depcrateimpl_109 {
() => {
// Module: crate
// Provides: {"impl_109"}
// Dependencies: {}
impl TryFrom < OsString > for Utf8PathBuf { type Error = FromOsStringError ; fn try_from (os_string : OsString) -> Result < Utf8PathBuf , Self :: Error > { Utf8PathBuf :: from_os_string (os_string) . map_err (| os_string | FromOsStringError { os_string , error : FromOsStrError (()) , }) } }
};
}
