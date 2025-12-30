// Generated macro for joining_type_to_mask (function)
macro_rules! Depcratejoining_type_to_mask {
() => {
// Module: crate
// Provides: {"joining_type_to_mask"}
// Dependencies: {}
# [doc = " Turns a joining type into a mask for comparing with multiple type at once."] const fn joining_type_to_mask (jt : icu_properties :: props :: JoiningType) -> u32 { 1u32 << jt . to_icu4c_value () }
};
}
