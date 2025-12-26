use lib_cargo::define_root_cargo_toml;
use lib_cargo::CargoToml;
use toml;

fn main() -> anyhow::Result<()> { // Change return type to Result
    let generated_cargo_toml: CargoToml = define_root_cargo_toml! {
        [package] {
            mkbuildrs!()
        }
    };
    
    let toml_string = toml::to_string_pretty(&generated_cargo_toml)
        .expect("Failed to serialize CargoToml to TOML string");
    
    println!("{}", toml_string);
    
    Ok(()) // Return Ok
}
