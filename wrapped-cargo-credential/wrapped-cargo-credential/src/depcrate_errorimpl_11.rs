// Generated macro for impl_11 (impl)
macro_rules! Depcrate_errorimpl_11 {
() => {
// Module: crate::error
// Provides: {"impl_11"}
// Dependencies: {}
impl From < anyhow :: Error > for Error { fn from (value : anyhow :: Error) -> Self { let mut prev = None ; for e in value . chain () . rev () { prev = Some (Box :: new (StringTypedError { message : e . to_string () , source : prev , })) ; } Error :: Other (prev . unwrap ()) } }
};
}
