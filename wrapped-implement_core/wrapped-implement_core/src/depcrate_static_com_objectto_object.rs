// Generated macro for to_object (function)
macro_rules! Depcrate_static_com_objectto_object {
() => {
// Module: crate::static_com_object
// Provides: {"to_object"}
// Dependencies: {}
# [test] fn to_object () { let factory_outer : & MyFactory_Impl = NUMBER_FACTORY_INSTANCE . get () ; let factory_object : ComObject < MyFactory > = factory_outer . to_object () ; assert_eq ! (unsafe { factory_object . add (333 , 444) } , 777) ; }
};
}
