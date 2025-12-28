macro_rules! RenameConfig {
    () => {
        pub struct RenameConfig { pub prefer_no_std : bool , pub prefer_prelude : bool , pub prefer_absolute : bool , }
    };
}

RenameConfig!()