// Generated macro for maybe_link_cabi_realloc (function)
macro_rules! Depcrate_rtmaybe_link_cabi_realloc {
() => {
// Module: crate::rt
// Provides: {"maybe_link_cabi_realloc"}
// Dependencies: {}
# [doc = " This function is called from generated bindings and will be deleted by"] # [doc = " the linker. The purpose of this function is to force a reference to the"] # [doc = " symbol `cabi_realloc` to make its way through to the final linker"] # [doc = " command line. That way `wasm-ld` will pick it up, see it needs to be"] # [doc = " exported, and then export it."] # [doc = ""] # [doc = " For more information about this see `./ci/rebuild-libwit-bindgen-cabi.sh`."] pub fn maybe_link_cabi_realloc () { # [cfg (all (target_family = "wasm" , not (target_env = "p2")))] { extern "C" { fn cabi_realloc (old_ptr : * mut u8 , old_len : usize , align : usize , new_len : usize ,) -> * mut u8 ; } # [used] static _NAME_DOES_NOT_MATTER : unsafe extern "C" fn (* mut u8 , usize , usize , usize ,) -> * mut u8 = cabi_realloc ; } }
};
}
