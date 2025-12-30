// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; macro_rules ! create_test { ($ name : ident , $ cargo_toml : expr , $ workspace_toml : expr , $ ($ result : tt) *) => { # [test] fn $ name () { let cargo_toml = $ cargo_toml . parse ::< DocumentMut > () . expect ("Parses `Cargo.toml`") ; let workspace_cargo_toml = $ workspace_toml . parse ::< DocumentMut > () . expect ("Parses workspace `Cargo.toml`") ; let workspace_deps = extract_workspace_dependencies (& workspace_cargo_toml) . expect ("Extracts workspace dependencies") ; match extract_crate_names (& cargo_toml , workspace_deps) . map (| mut map | map . remove ("my_crate")) { $ ($ result) * => () , o => panic ! ("Invalid result: {:?}" , o) , } } } ; } create_test ! { deps_with_crate , r#"
            [dependencies]
            my_crate = "0.1"
        "# , "" , Ok (Some (FoundCrate :: Name (name))) if name == "my_crate" } create_test ! { deps_with_crate_inline_table , r#"
            dependencies = { my_crate = "0.1" }
        "# , "" , Ok (Some (FoundCrate :: Name (name))) if name == "my_crate" } create_test ! { dev_deps_with_crate , r#"
            [dev-dependencies]
            my_crate = "0.1"
        "# , "" , Ok (Some (FoundCrate :: Name (name))) if name == "my_crate" } create_test ! { deps_with_crate_renamed , r#"
            [dependencies]
            cool = { package = "my_crate", version = "0.1" }
        "# , "" , Ok (Some (FoundCrate :: Name (name))) if name == "cool" } create_test ! { deps_with_crate_renamed_second , r#"
            [dependencies.cool]
            package = "my_crate"
            version = "0.1"
        "# , "" , Ok (Some (FoundCrate :: Name (name))) if name == "cool" } create_test ! { deps_empty , r#"
            [dependencies]
        "# , "" , Ok (None) } create_test ! { crate_not_found , r#"
            [dependencies]
            serde = "1.0"
        "# , "" , Ok (None) } create_test ! { target_dependency , r#"
            [target.'cfg(target_os="android")'.dependencies]
            my_crate = "0.1"
        "# , "" , Ok (Some (FoundCrate :: Name (name))) if name == "my_crate" } create_test ! { target_dependency2 , r#"
            [target.x86_64-pc-windows-gnu.dependencies]
            my_crate = "0.1"
        "# , "" , Ok (Some (FoundCrate :: Name (name))) if name == "my_crate" } create_test ! { own_crate , r#"
            [package]
            name = "my_crate"
        "# , "" , Ok (Some (FoundCrate :: Itself)) } create_test ! { own_crate_and_in_deps , r#"
            [package]
            name = "my_crate"

            [dev-dependencies]
            my_crate = "0.1"
        "# , "" , Ok (Some (FoundCrate :: Itself)) } create_test ! { multiple_times , r#"
            [dependencies]
            my_crate = { version = "0.5" }
            my-crate-old = { package = "my_crate", version = "0.1" }
        "# , "" , Ok (Some (FoundCrate :: Name (name))) if name == "my_crate_old" } create_test ! { workspace_deps , r#"
            [dependencies]
            my_crate_cool = { workspace = true }
        "# , r#"
            [workspace.dependencies]
            my_crate_cool = { package = "my_crate" }
        "# , Ok (Some (FoundCrate :: Name (name))) if name == "my_crate_cool" } }
};
}
