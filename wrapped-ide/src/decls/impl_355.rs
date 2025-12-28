macro_rules! deps {
    () => {
        RenameConfig!();
    };
}

macro_rules! impl_355 {
    () => {
        deps!();
        impl RenameConfig { fn find_path_config (& self) -> FindPathConfig { FindPathConfig { prefer_no_std : self . prefer_no_std , prefer_prelude : self . prefer_prelude , prefer_absolute : self . prefer_absolute , allow_unstable : true , } } }
    };
}

impl_355!()