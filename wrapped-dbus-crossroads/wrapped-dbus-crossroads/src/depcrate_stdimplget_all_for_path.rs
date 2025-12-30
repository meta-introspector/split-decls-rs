// Generated macro for get_all_for_path (function)
macro_rules! Depcrate_stdimplget_all_for_path {
() => {
// Module: crate::stdimpl
// Provides: {"get_all_for_path"}
// Dependencies: {}
fn get_all_for_path < F > (path : & dbus :: Path < 'static > , cr : & mut Crossroads , octx : Option < Context > , f : F) -> Option < Context > where F : FnOnce (& mut IfaceContext , & mut Option < Context >) + Send + 'static { let (_reg , ifaces) = cr . registry_and_ifaces (& path) ; let all : Vec < usize > = ifaces . into_iter () . map (| token | * token) . collect () ; for_each_interface_with_properties (path , all , cr , octx , f) }
};
}
