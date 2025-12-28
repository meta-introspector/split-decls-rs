macro_rules! deps {
    () => {
        RealCargoTomlParser!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        # [cfg (feature = "toml_edit_enabled")] impl CargoTomlParser for RealCargoTomlParser { fn get_package_repository (& self , path : & Path) -> Result < Option < String > , String > { let cargo_toml_content = fs :: read_to_string (path) . map_err (| e | format ! ("Failed to read Cargo.toml from {:?}: {}" , path , e)) ? ; let doc = cargo_toml_content . parse :: < Document < _ > > () . map_err (| e | format ! ("Failed to parse Cargo.toml: {}" , e)) ? ; let repo_url = doc . get ("package") . and_then (Item :: as_table) . and_then (| p | p . get ("repository")) . and_then (Item :: as_str) . map (| s | s . to_string ()) ; Ok (repo_url) } }
    };
}

impl_28!();