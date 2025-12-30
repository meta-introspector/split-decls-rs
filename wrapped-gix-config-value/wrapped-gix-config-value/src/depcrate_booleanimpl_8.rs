// Generated macro for impl_8 (impl)
macro_rules! Depcrate_booleanimpl_8 {
() => {
// Module: crate::boolean
// Provides: {"impl_8"}
// Dependencies: {}
impl TryFrom < OsString > for Boolean { type Error = Error ; fn try_from (value : OsString) -> Result < Self , Self :: Error > { let value = gix_path :: os_str_into_bstr (& value) . map_err (| _ | Error :: new ("Illformed UTF-8" , std :: path :: Path :: new (& value) . display () . to_string ())) ? ; Self :: try_from (value) } }
};
}
