macro_rules! RustcToolInfoTrait {
    () => {
        pub trait RustcToolInfoTrait : Send + Sync + Debug { fn invocation_method (& self) -> Option < & str > ; fn rustc_path (& self) -> Option < & str > ; fn cargo_path (& self) -> Option < & str > ; fn target_triple (& self) -> Option < & str > ; fn sysroot (& self) -> Option < & str > ; }
    };
}

RustcToolInfoTrait!();