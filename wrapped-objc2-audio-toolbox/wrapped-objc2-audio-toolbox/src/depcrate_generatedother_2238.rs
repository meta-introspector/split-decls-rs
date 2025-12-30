// Generated macro for other_2238 (other)
macro_rules! Depcrate_generatedother_2238 {
() => {
// Module: crate::generated
// Provides: {"other_2238"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Gets information about a clock's property."] # [doc = ""] # [doc = ""] # [doc = " Parameter `inCAClock`: The clock object."] # [doc = ""] # [doc = ""] # [doc = " Parameter `inPropertyID`: The property being queried."] # [doc = ""] # [doc = ""] # [doc = " Parameter `outSize`: If non-null, on exit, this is set to the size of the"] # [doc = " property's value."] # [doc = ""] # [doc = ""] # [doc = " Parameter `outWritable`: If non-null, on exit, this indicates whether the"] # [doc = " property value is settable."] # [doc = ""] # [doc = ""] # [doc = " Returns: An OSStatus error code."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `in_ca_clock` must be a valid pointer."] # [doc = " - `out_size` must be a valid pointer or null."] # [doc = " - `out_writable` must be a valid pointer or null."] pub fn CAClockGetPropertyInfo (in_ca_clock : CAClockRef , in_property_id : CAClockPropertyID , out_size : * mut u32 , out_writable : * mut Boolean ,) -> OSStatus ; }
};
}
