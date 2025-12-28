macro_rules! deps {
    () => {
        Metadata!();
        Package!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl Metadata { # [doc = " Get the workspace's root package of this metadata instance."] pub fn root_package (& self) -> Option < & Package > { match & self . resolve { Some (resolve) => { let root = resolve . root . as_ref () ? ; self . packages . iter () . find (| pkg | & pkg . id == root) } None => { let root_manifest_path = self . workspace_root . join ("Cargo.toml") ; self . packages . iter () . find (| pkg | pkg . manifest_path == root_manifest_path) } } } # [doc = " Get the workspace packages."] pub fn workspace_packages (& self) -> Vec < & Package > { self . packages . iter () . filter (| & p | self . workspace_members . contains (& p . id)) . collect () } # [doc = " Get the workspace default packages."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This will panic if running with a version of Cargo older than 1.71."] pub fn workspace_default_packages (& self) -> Vec < & Package > { self . packages . iter () . filter (| & p | self . workspace_default_members . contains (& p . id)) . collect () } }
    };
}

impl_12!()