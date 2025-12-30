// Generated macro for impl_42 (impl)
macro_rules! Depcrate_errorimpl_42 {
() => {
// Module: crate::error
// Provides: {"impl_42"}
// Dependencies: {}
impl PartialEq for ServerError { fn eq (& self , other : & Self) -> bool { self . message . eq (& other . message) && self . locations . eq (& other . locations) && self . path . eq (& other . path) && self . extensions . eq (& other . extensions) } }
};
}
