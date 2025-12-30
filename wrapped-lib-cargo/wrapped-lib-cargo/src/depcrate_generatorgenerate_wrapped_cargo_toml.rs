// Generated macro for generate_wrapped_cargo_toml (function)
macro_rules! Depcrate_generatorgenerate_wrapped_cargo_toml {
() => {
// Module: crate::generator
// Provides: {"generate_wrapped_cargo_toml"}
// Dependencies: {}
# [doc = " Generate a new Cargo.toml for a wrapped crate"] pub fn generate_wrapped_cargo_toml (original_cargo_path : & Path , output_cargo_path : & Path , crate_name : & str ,) -> Result < () > { let mut manifest = crate :: manifest :: read_manifest (original_cargo_path) ? ; if let Some (package) = manifest . get_mut ("package") { if let Some (package_table) = package . as_table_mut () { package_table . insert ("name" . to_string () , Value :: String (format ! ("wrapped-{}" , crate_name))) ; } } crate :: manifest :: update_edition (& mut manifest , "2024") ? ; convert_deps_to_workspace (& mut manifest) ? ; crate :: manifest :: write_manifest (output_cargo_path , & manifest) ? ; Ok (()) }
};
}
