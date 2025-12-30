// Generated macro for other_2239 (other)
macro_rules! Depcrate_generatedother_2239 {
() => {
// Module: crate::generated
// Provides: {"other_2239"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Gets the current value of a clock's property."] # [doc = ""] # [doc = ""] # [doc = " Parameter `inCAClock`: The clock object."] # [doc = ""] # [doc = ""] # [doc = " Parameter `inPropertyID`: The property being fetched."] # [doc = ""] # [doc = ""] # [doc = " Parameter `ioPropertyDataSize`: On entry, the size (in bytes) of the memory pointed to"] # [doc = " by outPropertyData. On exit, the actual size of the"] # [doc = " property data returned."] # [doc = ""] # [doc = ""] # [doc = " Parameter `outPropertyData`: The value of the property is copied to the memory"] # [doc = " this points to."] # [doc = ""] # [doc = ""] # [doc = " Returns: An OSStatus error code."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `in_ca_clock` must be a valid pointer."] # [doc = " - `io_property_data_size` must be a valid pointer."] # [doc = " - `out_property_data` must be a valid pointer."] pub fn CAClockGetProperty (in_ca_clock : CAClockRef , in_property_id : CAClockPropertyID , io_property_data_size : NonNull < u32 > , out_property_data : NonNull < c_void > ,) -> OSStatus ; }
};
}
