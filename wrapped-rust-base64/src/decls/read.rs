macro_rules! read {
    () => {
        # [cfg (any (feature = "std" , test))] pub mod read ;
    };
}

read!()