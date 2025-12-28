macro_rules! deps {
    () => {
        SysrootPublicDeps!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl SysrootPublicDeps { # [doc = " Makes `from` depend on the public sysroot crates."] fn add_to_crate_graph (& self , crate_graph : & mut CrateGraphBuilder , from : CrateBuilderId) { for (name , krate , prelude) in & self . deps { add_dep_with_prelude (crate_graph , from , name . clone () , * krate , * prelude , true) ; } } }
    };
}

impl_108!();