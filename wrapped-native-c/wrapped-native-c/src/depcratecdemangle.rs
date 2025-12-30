// Generated macro for CDemangle (struct)
macro_rules! DepcrateCDemangle {
() => {
// Module: crate
// Provides: {"CDemangle"}
// Dependencies: {}
# [doc = " struct demangle"] # [repr (C)] # [derive (Copy , Clone)] pub struct CDemangle { style : c_int , mangled : * const c_char , mangled_len : usize , elements : usize , original : * const c_char , original_len : usize , suffix : * const c_char , suffix_len : usize , }
};
}
