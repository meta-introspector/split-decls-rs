// Generated macro for from_inner_ref (function)
macro_rules! Depcrate_com_objectfrom_inner_ref {
() => {
// Module: crate::com_object
// Provides: {"from_inner_ref"}
// Dependencies: {}
# [test] fn from_inner_ref () { let app = MyApp :: new (42) ; let ifoo : InterfaceRef < IFoo > = app . as_interface () ; let ibar : IBar = unsafe { ifoo . get_self_as_bar () } ; unsafe { ibar . say_hello () } ; }
};
}
