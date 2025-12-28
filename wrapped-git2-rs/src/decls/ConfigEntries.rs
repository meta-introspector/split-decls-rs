macro_rules! deps {
    () => {
        Config!();
        ConfigEntry!();
    };
}

macro_rules! ConfigEntries {
    () => {
        deps!();
        # [doc = " An iterator over the `ConfigEntry` values of a `Config` structure."] # [doc = ""] # [doc = " Due to lifetime restrictions, `ConfigEntries` does not implement the"] # [doc = " standard [`Iterator`] trait. It provides a [`next`] function which only"] # [doc = " allows access to one entry at a time. [`for_each`] is available as a"] # [doc = " convenience function."] # [doc = ""] # [doc = " [`next`]: ConfigEntries::next"] # [doc = " [`for_each`]: ConfigEntries::for_each"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " // Example of how to collect all entries."] # [doc = " use git2::Config;"] # [doc = ""] # [doc = " let config = Config::new()?;"] # [doc = " let iter = config.entries(None)?;"] # [doc = " let mut entries = Vec::new();"] # [doc = " iter"] # [doc = "     .for_each(|entry| {"] # [doc = "         let name = entry.name().unwrap().to_string();"] # [doc = "         let value = entry.value().unwrap_or(\"\").to_string();"] # [doc = "         entries.push((name, value))"] # [doc = "     })?;"] # [doc = " for entry in &entries {"] # [doc = "     println!(\"{} = {}\", entry.0, entry.1);"] # [doc = " }"] # [doc = " # Ok::<(), git2::Error>(())"] # [doc = ""] # [doc = " ```"] pub struct ConfigEntries < 'cfg > { raw : * mut raw :: git_config_iterator , current : Option < ConfigEntry < 'cfg > > , _marker : marker :: PhantomData < & 'cfg Config > , }
    };
}

ConfigEntries!()