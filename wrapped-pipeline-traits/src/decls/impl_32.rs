macro_rules! deps {
    () => {
        NixDetails!();
        NixInfoTrait!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl NixInfoTrait for NixDetails { fn nix_flake_path (& self) -> Option < & str > { match self { NixDetails :: Info (info) => Some (& info . flake_path) , _ => None , } } fn nix_output_type (& self) -> Option < & str > { match self { NixDetails :: Info (info) => Some (& info . output_type) , _ => None , } } }
    };
}

impl_32!();