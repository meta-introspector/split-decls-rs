macro_rules! submodule {
    () => {
        # [cfg (feature = "attributes")] pub mod submodule ;
    };
}

submodule!();