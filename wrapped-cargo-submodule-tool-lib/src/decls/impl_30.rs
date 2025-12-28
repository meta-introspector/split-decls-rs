macro_rules! deps {
    () => {
        DummyCargoTomlParser!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        # [cfg (not (feature = "toml_edit_enabled"))] impl CargoTomlParser for DummyCargoTomlParser { fn get_package_repository (& self , _path : & Path) -> Result < Option < String > > { Ok (None) } }
    };
}

impl_30!()