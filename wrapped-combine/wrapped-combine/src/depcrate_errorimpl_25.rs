// Generated macro for impl_25 (impl)
macro_rules! Depcrate_errorimpl_25 {
() => {
// Module: crate::error
// Provides: {"impl_25"}
// Dependencies: {}
impl < 's , 'a , T , R , F > ErrorInfo < 's , T , R > for & 'a F where F : ErrorInfo < 's , T , R > , { type Format = F :: Format ; fn into_info (& 's self) -> Info < T , R , Self :: Format > { (* * self) . into_info () } }
};
}
