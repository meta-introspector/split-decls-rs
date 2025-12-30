// Generated macro for CargoToml (struct)
macro_rules! Depcrate_cargo_processorCargoToml {
() => {
// Module: crate::cargo_processor
// Provides: {"CargoToml"}
// Dependencies: {}
# [derive (Debug , Clone , Serialize , Deserialize)] pub struct CargoToml { pub package : PackageInfo , # [serde (default)] pub dependencies : HashMap < String , toml :: Value > , # [serde (default , rename = "dev-dependencies")] pub dev_dependencies : HashMap < String , toml :: Value > , # [serde (default , rename = "build-dependencies")] pub build_dependencies : HashMap < String , toml :: Value > , # [serde (default)] pub features : Option < HashMap < String , toml :: Value > > , # [serde (default)] pub workspace : Option < toml :: Value > , # [serde (default)] pub profile : Option < toml :: Value > , # [serde (flatten)] pub other : HashMap < String , toml :: Value > , }
};
}
