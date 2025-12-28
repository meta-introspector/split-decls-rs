macro_rules! deps {
    () => {
        Cargo2NixCommand!();
        RemoveRustVersionCommand!();
        CargoUpdateCommand!();
        CargoCommand!();
        CargoVendorCommand!();
    };
}

macro_rules! get_cargo_command {
    () => {
        deps!();
        pub fn get_cargo_command (command_str : & str) -> Option < Box < dyn CargoCommand > > { match command_str { "cargo update" => Some (Box :: new (super :: cargo_update_command :: CargoUpdateCommand)) , "cargo vendor" => Some (Box :: new (super :: cargo_vendor_command :: CargoVendorCommand)) , "cargo2nix" => Some (Box :: new (super :: cargo2nix_command :: Cargo2NixCommand)) , "remove rust version constraints" => Some (Box :: new (super :: remove_rust_version_command :: RemoveRustVersionCommand ,)) , _ => None , } }
    };
}

get_cargo_command!();