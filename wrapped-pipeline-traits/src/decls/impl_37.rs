macro_rules! deps {
    () => {
        RustDetailsInfoTrait!();
        RustDetails!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl RustDetailsInfoTrait for RustDetails { fn version (& self) -> Option < & str > { match self { RustDetails :: Info (info) => Some (& info . version) , _ => None , } } fn crate_name (& self) -> Option < & str > { match self { RustDetails :: Info (info) => Some (& info . crate_name) , _ => None , } } fn item_path (& self) -> Option < & str > { match self { RustDetails :: Info (info) => Some (& info . item_path) , _ => None , } } }
    };
}

impl_37!();