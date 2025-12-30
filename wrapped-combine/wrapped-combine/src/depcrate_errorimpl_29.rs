// Generated macro for impl_29 (impl)
macro_rules! Depcrate_errorimpl_29 {
() => {
// Module: crate::error
// Provides: {"impl_29"}
// Dependencies: {}
impl < 's , R > ErrorInfo < 's , char , R > for char { type Format = & 'static str ; fn into_info (& self) -> Info < char , R , Self :: Format > { Info :: Token (* self) } }
};
}
