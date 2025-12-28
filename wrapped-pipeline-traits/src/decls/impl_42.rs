macro_rules! deps {
    () => {
        RustcToolInfoTrait!();
        RustcToolDetails!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl RustcToolInfoTrait for RustcToolDetails { fn invocation_method (& self) -> Option < & str > { match self { RustcToolDetails :: Info (info) => Some (& info . invocation_method) , _ => None , } } fn rustc_path (& self) -> Option < & str > { match self { RustcToolDetails :: Info (info) => info . rustc_path . as_deref () , _ => None , } } fn cargo_path (& self) -> Option < & str > { match self { RustcToolDetails :: Info (info) => info . cargo_path . as_deref () , _ => None , } } fn target_triple (& self) -> Option < & str > { match self { RustcToolDetails :: Info (info) => info . target_triple . as_deref () , _ => None , } } fn sysroot (& self) -> Option < & str > { match self { RustcToolDetails :: Info (info) => info . sysroot . as_deref () , _ => None , } } }
    };
}

impl_42!()