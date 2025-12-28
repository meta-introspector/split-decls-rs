macro_rules! negotiate {
    () => {
        # [cfg (feature = "fetch")] pub mod negotiate ;
    };
}

negotiate!()