// Generated macro for impl_261 (impl)
macro_rules! Depcrate_selectimpl_261 {
() => {
// Module: crate::select
// Provides: {"impl_261"}
// Dependencies: {}
impl From < usize > for Selected { # [inline] fn from (val : usize) -> Self { match val { 0 => Self :: Waiting , 1 => Self :: Aborted , 2 => Self :: Disconnected , oper => Self :: Operation (Operation (oper)) , } } }
};
}
