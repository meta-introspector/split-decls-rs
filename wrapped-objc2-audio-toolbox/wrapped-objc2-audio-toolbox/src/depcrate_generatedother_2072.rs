// Generated macro for other_2072 (other)
macro_rules! Depcrate_generatedother_2072 {
() => {
// Module: crate::generated
// Provides: {"other_2072"}
// Dependencies: {}
extern "C-unwind" { # [doc = " Queries an AudioObject to find the size of the data for the given property."] # [doc = ""] # [doc = " Parameter `inObjectID`: The AudioObject to query."] # [doc = ""] # [doc = " Parameter `inAddress`: An AudioObjectPropertyAddress indicating which property is being queried."] # [doc = ""] # [doc = " Parameter `inQualifierDataSize`: A UInt32 indicating the size of the buffer pointed to by inQualifierData."] # [doc = " Note that not all properties require qualification, in which case this"] # [doc = " value will be 0."] # [doc = ""] # [doc = " Parameter `inQualifierData`: A buffer of data to be used in determining the data of the property being"] # [doc = " queried. Note that not all properties require qualification, in which case"] # [doc = " this value will be NULL."] # [doc = ""] # [doc = " Parameter `outDataSize`: A UInt32 indicating how many bytes the data for the given property occupies."] # [doc = ""] # [doc = " Returns: An OSStatus indicating success or failure."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `in_address` must be a valid pointer."] # [doc = " - `in_qualifier_data` must be a valid pointer."] # [doc = " - `out_data_size` must be a valid pointer."] # [cfg (feature = "objc2-core-audio")] # [deprecated = "no longer supported"] pub fn AudioHardwareServiceGetPropertyDataSize (in_object_id : AudioObjectID , in_address : * const AudioObjectPropertyAddress , in_qualifier_data_size : u32 , in_qualifier_data : * const c_void , out_data_size : * mut u32 ,) -> OSStatus ; }
};
}
