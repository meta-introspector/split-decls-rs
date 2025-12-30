// Generated macro for ForeachCb (type)
macro_rules! Depcrate_odbForeachCb {
() => {
// Module: crate::odb
// Provides: {"ForeachCb"}
// Dependencies: {}
pub type ForeachCb < 'a > = dyn FnMut (& Oid) -> bool + 'a ;
};
}
