// Generated macro for write_ancestor (function)
macro_rules! Depcrate_blob_builtin_driver_text_utilswrite_ancestor {
() => {
// Module: crate::blob::builtin_driver::text::utils
// Provides: {"write_ancestor"}
// Dependencies: {}
pub fn write_ancestor (input : & InternedInput < & [u8] > , from : u32 , to : usize , out : & mut Vec < u8 >) { if to < from as usize { return ; } if let Some (tokens) = input . before . get (from as usize .. to) { write_tokens (& input . interner , tokens , out) ; } }
};
}
