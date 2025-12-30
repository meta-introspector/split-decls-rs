// Generated macro for get_string (function)
macro_rules! Depcrate_descriptorget_string {
() => {
// Module: crate::descriptor
// Provides: {"get_string"}
// Dependencies: {}
fn get_string (data : & mut & [u32]) -> String { (0 .. get (data)) . map (| _ | char :: from_u32 (get (data)) . unwrap ()) . collect () }
};
}
