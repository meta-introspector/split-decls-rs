macro_rules! Mode {
    () => {
        pub (crate) enum Mode { DeletedPacksAreInaccessible , # [doc = " This mode signals that we should not unload packs even after they went missing."] KeepDeletedPacksAvailable , }
    };
}

Mode!()