// Generated macro for Settings (struct)
macro_rules! Depcrate_settingsSettings {
() => {
// Module: crate::settings
// Provides: {"Settings"}
// Dependencies: {}
# [doc = " Configures how insta operates at test time."] # [doc = ""] # [doc = " Settings are always bound to a thread, and some default settings are always"] # [doc = " available.  These settings can be changed and influence how insta behaves on"] # [doc = " that thread.  They can be either temporarily or permanently changed."] # [doc = ""] # [doc = " This can be used to influence how the snapshot macros operate."] # [doc = " For instance, it can be useful to force ordering of maps when"] # [doc = " unordered structures are used through settings."] # [doc = ""] # [doc = " Some of the settings can be changed but shouldn't as it will make it harder"] # [doc = " for tools like cargo-insta or an editor integration to locate the snapshot"] # [doc = " files."] # [doc = ""] # [doc = " Settings can also be configured with the [`with_settings!`] macro."] # [doc = ""] # [doc = " Example:"] # [doc = ""] # [doc = " ```ignore"] # [doc = " use insta;"] # [doc = ""] # [doc = " let mut settings = insta::Settings::clone_current();"] # [doc = " settings.set_sort_maps(true);"] # [doc = " settings.bind(|| {"] # [doc = "     // runs the assertion with the changed settings enabled"] # [doc = "     insta::assert_snapshot!(...);"] # [doc = " });"] # [doc = " ```"] # [derive (Clone)] pub struct Settings { inner : Arc < ActualSettings > , }
};
}
