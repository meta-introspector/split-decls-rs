// Generated macro for to_interface (function)
macro_rules! Depcrate_static_com_objectto_interface {
() => {
// Module: crate::static_com_object
// Provides: {"to_interface"}
// Dependencies: {}
# [test] fn to_interface () { let factory_outer : & MyFactory_Impl = NUMBER_FACTORY_INSTANCE . get () ; let ifactory : INumberFactory = factory_outer . to_interface :: < INumberFactory > () ; assert_eq ! (unsafe { ifactory . add (333 , 444) } , 777) ; drop (ifactory) ; }
};
}
