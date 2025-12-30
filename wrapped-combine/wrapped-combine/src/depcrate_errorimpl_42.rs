// Generated macro for impl_42 (impl)
macro_rules! Depcrate_errorimpl_42 {
() => {
// Module: crate::error
// Provides: {"impl_42"}
// Dependencies: {}
impl < 's , T , R > ErrorInfo < 's , T , R > for Static { type Format = & 'static str ; fn into_info (& 's self) -> Info < T , R , Self :: Format > { Info :: Static (self . 0) } }
};
}
