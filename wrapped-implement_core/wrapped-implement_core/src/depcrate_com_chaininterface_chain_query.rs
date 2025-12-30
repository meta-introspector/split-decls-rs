// Generated macro for interface_chain_query (function)
macro_rules! Depcrate_com_chaininterface_chain_query {
() => {
// Module: crate::com_chain
// Provides: {"interface_chain_query"}
// Dependencies: {}
# [test] fn interface_chain_query () { let object = ComObject :: new (ObjectWithChains { }) ; let unknown : IUnknown = object . to_interface () ; let _foo : IFoo = unknown . cast () . expect ("QueryInterface for IFoo") ; let _foo2 : IFoo2 = unknown . cast () . expect ("QueryInterface for IFoo2") ; let _foo3 : IFoo3 = unknown . cast () . expect ("QueryInterface for IFoo3") ; }
};
}
