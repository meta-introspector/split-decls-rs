macro_rules! deps {
    () => {
        LinuxInfoTrait!();
        LinuxDetails!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl LinuxInfoTrait for LinuxDetails { fn kernel_version (& self) -> Option < & str > { match self { LinuxDetails :: Info (info) => Some (& info . kernel_version) , _ => None , } } fn architecture (& self) -> Option < & str > { match self { LinuxDetails :: Info (info) => Some (& info . architecture) , _ => None , } } }
    };
}

impl_22!();