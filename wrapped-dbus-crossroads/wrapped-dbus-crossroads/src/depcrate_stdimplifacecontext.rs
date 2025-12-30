// Generated macro for IfaceContext (struct)
macro_rules! Depcrate_stdimplIfaceContext {
() => {
// Module: crate::stdimpl
// Provides: {"IfaceContext"}
// Dependencies: {}
# [derive (Debug , Default)] struct IfaceContext { remaining : usize , ifaces : IfacePropMap , donefn : Option < Dbg < Box < dyn FnOnce (& mut IfaceContext , & mut Option < Context >) + Send + 'static > > > , }
};
}
