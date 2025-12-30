// Generated macro for impl_31 (impl)
macro_rules! Depcrate_errorimpl_31 {
() => {
// Module: crate::error
// Provides: {"impl_31"}
// Dependencies: {}
impl < 's , T , R > ErrorInfo < 's , T , R > for & 'static str { type Format = & 'static str ; fn into_info (& self) -> Info < T , R , Self :: Format > { Info :: Static (* self) } }
};
}
