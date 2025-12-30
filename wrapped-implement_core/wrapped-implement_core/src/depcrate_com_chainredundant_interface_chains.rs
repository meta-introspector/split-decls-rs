// Generated macro for redundant_interface_chains (function)
macro_rules! Depcrate_com_chainredundant_interface_chains {
() => {
// Module: crate::com_chain
// Provides: {"redundant_interface_chains"}
// Dependencies: {}
# [test] fn redundant_interface_chains () { let object = ComObject :: new (ObjectRedundantChains { }) ; let unknown : IUnknown = object . to_interface () ; let _foo : IFoo = unknown . cast () . expect ("QueryInterface for IFoo") ; let _foo2 : IFoo2 = unknown . cast () . expect ("QueryInterface for IFoo2") ; let _foo3 : IFoo3 = unknown . cast () . expect ("QueryInterface for IFoo3") ; }
};
}
