// Generated macro for value_for_array (function)
macro_rules! Depcrate_common_valuesvalue_for_array {
() => {
// Module: crate::common::values
// Provides: {"value_for_array"}
// Dependencies: {}
# [doc = " Get a single value for an argument values array in a determistic way."] # [doc = " * `bits`: The number of bits for the type, only 8, 16, 32, 64 are valid values"] # [doc = " * `index`: The position in the array we are generating for"] pub fn value_for_array (bits : u32 , index : u32) -> u64 { let index = index as usize ; match bits { 1 => VALUES_8 [index % 2] . into () , 2 => VALUES_8 [index % 4] . into () , 3 => VALUES_8 [index % 8] . into () , 4 => VALUES_8 [index % 16] . into () , 5 => VALUES_5 [index % VALUES_5 . len ()] . into () , 6 => VALUES_6 [index % VALUES_6 . len ()] . into () , 7 => VALUES_7 [index % VALUES_7 . len ()] . into () , 8 => VALUES_8 [index % VALUES_8 . len ()] . into () , 16 => VALUES_16 [index % VALUES_16 . len ()] . into () , 32 => VALUES_32 [index % VALUES_32 . len ()] . into () , 64 => VALUES_64 [index % VALUES_64 . len ()] , _ => unimplemented ! ("value_for_array(bits: {bits}, ..)") , } }
};
}
