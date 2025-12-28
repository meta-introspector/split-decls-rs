macro_rules! deps {
    () => {
        CargoDetails!();
        CargoInfoTrait!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl CargoInfoTrait for CargoDetails { fn package_name (& self) -> Option < & str > { match self { CargoDetails :: Info (info) => Some (& info . package_name) , _ => None , } } fn version (& self) -> Option < & str > { match self { CargoDetails :: Info (info) => Some (& info . version) , _ => None , } } }
    };
}

impl_12!()