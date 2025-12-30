// Generated macro for guest_export_needs_post_return (function)
macro_rules! Depcrate_abiguest_export_needs_post_return {
() => {
// Module: crate::abi
// Provides: {"guest_export_needs_post_return"}
// Dependencies: {}
# [doc = " Returns whether the `Function` specified needs a post-return function to"] # [doc = " be generated in guest code."] # [doc = ""] # [doc = " This is used when the return value contains a memory allocation such as"] # [doc = " a list or a string primarily."] pub fn guest_export_needs_post_return (resolve : & Resolve , func : & Function) -> bool { func . result . map (| t | needs_deallocate (resolve , & t , Deallocate :: Lists)) . unwrap_or (false) }
};
}
