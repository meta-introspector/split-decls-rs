macro_rules! deps {
    () => {
        TempCrateBuilder!();
    };
}

macro_rules! impl_239 {
    () => {
        deps!();
        impl TempCrateBuilder { pub async fn build_temp_crate (_file_path : & Path , _manifest_path : & Path ,) -> Result < (TempDir , PathBuf) > { let temp_dir = tempfile :: tempdir () ? ; let temp_cargo_toml_path = temp_dir . path () . join ("Cargo.toml") ; Ok ((temp_dir , temp_cargo_toml_path)) } }
    };
}

impl_239!()