// Generated macro for other_2240 (other)
macro_rules! Depcrate_generatedother_2240 {
() => {
// Module: crate::generated
// Provides: {"other_2240"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Changes the value of a clock's property."] # [doc = ""] # [doc = ""] # [doc = " Parameter `inCAClock`: The clock object."] # [doc = ""] # [doc = ""] # [doc = " Parameter `inPropertyID`: The property being set."] # [doc = ""] # [doc = ""] # [doc = " Parameter `inPropertyDataSize`: The size of the property data, in bytes."] # [doc = ""] # [doc = ""] # [doc = " Parameter `inPropertyData`: Points to the property's new value."] # [doc = ""] # [doc = ""] # [doc = " Returns: An OSStatus error code."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `in_ca_clock` must be a valid pointer."] # [doc = " - `in_property_data` must be a valid pointer."] pub fn CAClockSetProperty (in_ca_clock : CAClockRef , in_property_id : CAClockPropertyID , in_property_data_size : u32 , in_property_data : NonNull < c_void > ,) -> OSStatus ; }
};
}
