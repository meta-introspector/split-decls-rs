// Generated macro for macro_139 (macro)
macro_rules! Depcrate_objectmacro_139 {
() => {
// Module: crate::object
// Provides: {"macro_139"}
// Dependencies: {}
enum_with_val ! { # [doc = " Quality-of-service classes that specify the priorities for executing tasks."] # [doc (alias = "dispatch_qos_class_t")] # [derive (Copy , Clone , PartialEq , Eq , Hash , PartialOrd , Ord)] pub struct DispatchQoS (pub c_uint) { # [doc = " Quality of service for user-interactive tasks."] # [doc (alias = "QOS_CLASS_USER_INTERACTIVE")] UserInteractive = 0x21 , # [doc = " Quality of service for tasks that prevent the user from actively using your app."] # [doc (alias = "QOS_CLASS_USER_INITIATED")] UserInitiated = 0x19 , # [doc = " Default Quality of service."] # [doc (alias = "QOS_CLASS_DEFAULT")] Default = 0x15 , # [doc = " Quality of service for tasks that the user does not track actively."] # [doc (alias = "QOS_CLASS_UTILITY")] Utility = 0x11 , # [doc = " Quality of service for maintenance or cleanup tasks."] # [doc (alias = "QOS_CLASS_BACKGROUND")] Background = 0x09 , # [doc = " The absence of a Quality of service."] # [doc (alias = "QOS_CLASS_UNSPECIFIED")] Unspecified = 0x00 , } }
};
}
