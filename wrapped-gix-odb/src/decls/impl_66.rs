macro_rules! deps {
    () => {
        Handle!();
        RefreshMode!();
        Store!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl < S > super :: Handle < S > where S : Deref < Target = super :: Store > + Clone , { # [doc = " Call once if pack ids are stored and later used for lookup, meaning they should always remain mapped and not be unloaded"] # [doc = " even if they disappear from disk."] # [doc = " This must be called if there is a chance that git maintenance is happening while a pack is created."] pub fn prevent_pack_unload (& mut self) { self . token = self . token . take () . map (| token | self . store . upgrade_handle (token)) ; } # [doc = " Return a shared reference to the contained store."] pub fn store_ref (& self) -> & S :: Target { & self . store } # [doc = " Return an owned store with shared ownership."] pub fn store (& self) -> S { self . store . clone () } # [doc = " Set the handle to never cause ODB refreshes if an object could not be found."] # [doc = ""] # [doc = " The latter is the default, as typically all objects referenced in a git repository are contained in the local clone."] # [doc = " More recently, however, this doesn't always have to be the case due to sparse checkouts and other ways to only have a"] # [doc = " limited amount of objects available locally."] pub fn refresh_never (& mut self) { self . refresh = RefreshMode :: Never ; } # [doc = " Return the current refresh mode."] pub fn refresh_mode (& mut self) -> RefreshMode { self . refresh } }
    };
}

impl_66!()