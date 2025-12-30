// Generated macro for impl_33 (impl)
macro_rules! Depcrate_errorimpl_33 {
() => {
// Module: crate::error
// Provides: {"impl_33"}
// Dependencies: {}
impl < R > ErrorInfo < '_ , Self , R > for u8 { type Format = & 'static str ; fn into_info (& self) -> Info < Self , R , Self :: Format > { Info :: Token (* self) } }
};
}
