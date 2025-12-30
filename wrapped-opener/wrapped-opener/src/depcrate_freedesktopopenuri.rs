// Generated macro for OpenURI (trait)
macro_rules! Depcrate_freedesktopOpenURI {
() => {
// Module: crate::freedesktop
// Provides: {"OpenURI"}
// Dependencies: {}
# [doc = " # D-Bus interface proxy for: `org.freedesktop.portal.OpenURI`"] # [zbus :: proxy (gen_async = false , interface = "org.freedesktop.portal.OpenURI" , default_service = "org.freedesktop.portal.Desktop" , default_path = "/org/freedesktop/portal/desktop")] pub trait OpenURI { # [doc = " OpenDirectory method"] fn open_directory (& self , parent_window : & str , fd : zbus :: zvariant :: Fd < '_ > , options : HashMap < & str , & zbus :: zvariant :: Value < '_ > > ,) -> zbus :: Result < zbus :: zvariant :: OwnedObjectPath > ; }
};
}
