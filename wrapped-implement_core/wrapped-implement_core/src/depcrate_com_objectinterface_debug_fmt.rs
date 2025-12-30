// Generated macro for interface_debug_fmt (function)
macro_rules! Depcrate_com_objectinterface_debug_fmt {
() => {
// Module: crate::com_object
// Provides: {"interface_debug_fmt"}
// Dependencies: {}
# [test] fn interface_debug_fmt () { let app = MyApp :: new (42) ; let iunknown : IUnknown = app . to_interface () ; let unknown_dbg = format ! ("{iunknown:?}") ; assert ! (unknown_dbg . starts_with ("IUnknown(0x") , "{unknown_dbg:?}") ; let ifoo : IFoo = app . to_interface () ; let foo_dbg = format ! ("{ifoo:?}") ; assert ! (foo_dbg . starts_with ("IFoo(0x") , "{foo_dbg:?}") ; }
};
}
