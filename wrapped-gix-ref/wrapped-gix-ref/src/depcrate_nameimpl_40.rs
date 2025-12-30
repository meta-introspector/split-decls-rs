// Generated macro for impl_40 (impl)
macro_rules! Depcrate_nameimpl_40 {
() => {
// Module: crate::name
// Provides: {"impl_40"}
// Dependencies: {}
impl < 'a > convert :: TryFrom < & 'a OsStr > for & 'a PartialNameRef { type Error = Error ; fn try_from (v : & 'a OsStr) -> Result < Self , Self :: Error > { let v = gix_path :: os_str_into_bstr (v) . map_err (| _ | { Error :: Tag (gix_validate :: tag :: name :: Error :: InvalidByte { byte : "<unknown encoding>" . into () , }) }) ? ; Ok (PartialNameRef :: new_unchecked (gix_validate :: reference :: name_partial (v . as_bstr () ,) ?)) } }
};
}
