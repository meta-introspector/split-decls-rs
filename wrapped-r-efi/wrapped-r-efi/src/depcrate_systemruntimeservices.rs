// Generated macro for RuntimeServices (struct)
macro_rules! Depcrate_systemRuntimeServices {
() => {
// Module: crate::system
// Provides: {"RuntimeServices"}
// Dependencies: {}
# [repr (C)] pub struct RuntimeServices { pub hdr : TableHeader , pub get_time : RuntimeGetTime , pub set_time : RuntimeSetTime , pub get_wakeup_time : RuntimeGetWakeupTime , pub set_wakeup_time : RuntimeSetWakeupTime , pub set_virtual_address_map : RuntimeSetVirtualAddressMap , pub convert_pointer : RuntimeConvertPointer , pub get_variable : RuntimeGetVariable , pub get_next_variable_name : RuntimeGetNextVariableName , pub set_variable : RuntimeSetVariable , pub get_next_high_mono_count : RuntimeGetNextHighMonoCount , pub reset_system : RuntimeResetSystem , pub update_capsule : RuntimeUpdateCapsule , pub query_capsule_capabilities : RuntimeQueryCapsuleCapabilities , pub query_variable_info : RuntimeQueryVariableInfo , }
};
}
