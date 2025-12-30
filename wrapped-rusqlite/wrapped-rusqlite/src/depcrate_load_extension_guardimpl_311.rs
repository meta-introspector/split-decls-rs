// Generated macro for impl_311 (impl)
macro_rules! Depcrate_load_extension_guardimpl_311 {
() => {
// Module: crate::load_extension_guard
// Provides: {"impl_311"}
// Dependencies: {}
impl LoadExtensionGuard < '_ > { # [doc = " Attempt to enable loading extensions. Loading extensions will be"] # [doc = " disabled when this guard goes out of scope. Cannot be meaningfully"] # [doc = " nested."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " You must not run untrusted queries while extension loading is enabled."] # [doc = ""] # [doc = " See the safety comment on [`Connection::load_extension_enable`] for more"] # [doc = " details."] # [inline] pub unsafe fn new (conn : & Connection) -> Result < LoadExtensionGuard < '_ > > { conn . load_extension_enable () . map (| _ | LoadExtensionGuard { conn }) } }
};
}
