// Generated macro for as_interface (function)
macro_rules! Depcrate_static_com_objectas_interface {
() => {
// Module: crate::static_com_object
// Provides: {"as_interface"}
// Dependencies: {}
# [test] fn as_interface () { let factory_outer : & MyFactory_Impl = NUMBER_FACTORY_INSTANCE . get () ; let ifactory : InterfaceRef < INumberFactory > = factory_outer . as_interface :: < INumberFactory > () ; let n = unsafe { ifactory . next () } ; println ! ("n = {n:?}") ; assert_eq ! (unsafe { ifactory . add (333 , 444) } , 777) ; }
};
}
