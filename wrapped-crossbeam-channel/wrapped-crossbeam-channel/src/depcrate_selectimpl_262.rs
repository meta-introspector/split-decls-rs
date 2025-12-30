// Generated macro for impl_262 (impl)
macro_rules! Depcrate_selectimpl_262 {
() => {
// Module: crate::select
// Provides: {"impl_262"}
// Dependencies: {}
impl From < Selected > for usize { # [inline] fn from (val : Selected) -> Self { match val { Selected :: Waiting => 0 , Selected :: Aborted => 1 , Selected :: Disconnected => 2 , Selected :: Operation (Operation (val)) => val , } } }
};
}
