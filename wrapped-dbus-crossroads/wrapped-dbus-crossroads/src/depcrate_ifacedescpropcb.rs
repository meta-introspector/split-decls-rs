// Generated macro for PropCb (type)
macro_rules! Depcrate_ifacedescPropCb {
() => {
// Module: crate::ifacedesc
// Provides: {"PropCb"}
// Dependencies: {}
pub type PropCb = Box < dyn FnMut (PropContext , & mut Crossroads) -> Option < PropContext > + Send + 'static > ;
};
}
