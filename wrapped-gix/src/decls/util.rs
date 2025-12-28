macro_rules! util {
    () => {
        # [cfg (any (feature = "dirwalk" , feature = "status"))] pub (crate) mod util ;
    };
}

util!()