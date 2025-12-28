macro_rules! auto_extension {
    () => {
        # [cfg (not (feature = "loadable_extension"))] pub mod auto_extension ;
    };
}

auto_extension!()