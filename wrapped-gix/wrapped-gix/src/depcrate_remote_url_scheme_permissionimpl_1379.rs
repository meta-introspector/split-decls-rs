// Generated macro for impl_1379 (impl)
macro_rules! Depcrate_remote_url_scheme_permissionimpl_1379 {
() => {
// Module: crate::remote::url::scheme_permission
// Provides: {"impl_1379"}
// Dependencies: {}
impl < 'a > TryFrom < Cow < 'a , BStr > > for Allow { type Error = BString ; fn try_from (v : Cow < 'a , BStr >) -> Result < Self , Self :: Error > { Ok (match v . as_ref () . as_bytes () { b"never" => Allow :: Never , b"always" => Allow :: Always , b"user" => Allow :: User , unknown => return Err (unknown . into ()) , }) } }
};
}
