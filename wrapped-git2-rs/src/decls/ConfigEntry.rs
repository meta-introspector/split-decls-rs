macro_rules! deps {
    () => {
        Config!();
    };
}

macro_rules! ConfigEntry {
    () => {
        deps!();
        # [doc = " A struct representing a certain entry owned by a `Config` instance."] # [doc = ""] # [doc = " An entry has a name, a value, and a level it applies to."] pub struct ConfigEntry < 'cfg > { raw : * mut raw :: git_config_entry , _marker : marker :: PhantomData < & 'cfg Config > , owned : bool , }
    };
}

ConfigEntry!();