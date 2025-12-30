// Generated macro for Callback (type)
macro_rules! Depcrate_ifacedescCallback {
() => {
// Module: crate::ifacedesc
// Provides: {"Callback"}
// Dependencies: {}
pub type Callback = Box < dyn FnMut (Context , & mut Crossroads) -> Option < Context > + Send + 'static > ;
};
}
