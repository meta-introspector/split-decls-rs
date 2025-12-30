// Generated macro for ClassProtocolMethodsBuilder (struct)
macro_rules! Depcrate___macros_define_class_checksClassProtocolMethodsBuilder {
() => {
// Module: crate::__macros::define_class::checks
// Provides: {"ClassProtocolMethodsBuilder"}
// Dependencies: {}
# [doc = " Helper for ensuring that:"] # [doc = " - Only methods on the protocol are overridden."] # [doc = " - TODO: The methods have the correct signature."] # [doc = " - All required methods are overridden."] # [derive (Debug)] pub struct ClassProtocolMethodsBuilder < 'a , T : ? Sized > { builder : & 'a mut ClassBuilderHelper < T > , # [cfg (debug_assertions)] protocol : Option < & 'static AnyProtocol > , # [cfg (debug_assertions)] required_instance_methods : Vec < MethodDescription > , # [cfg (debug_assertions)] optional_instance_methods : Vec < MethodDescription > , # [cfg (debug_assertions)] registered_instance_methods : HashSet < Sel > , # [cfg (debug_assertions)] required_class_methods : Vec < MethodDescription > , # [cfg (debug_assertions)] optional_class_methods : Vec < MethodDescription > , # [cfg (debug_assertions)] registered_class_methods : HashSet < Sel > , }
};
}
