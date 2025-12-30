// Generated macro for ConfigEntry (struct)
macro_rules! Depcrate_configConfigEntry {
() => {
// Module: crate::config
// Provides: {"ConfigEntry"}
// Dependencies: {}
# [doc = " A struct representing a certain entry owned by a `Config` instance."] # [doc = ""] # [doc = " An entry has a name, a value, and a level it applies to."] pub struct ConfigEntry < 'cfg > { raw : * mut raw :: git_config_entry , _marker : marker :: PhantomData < & 'cfg Config > , owned : bool , }
};
}
