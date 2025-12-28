macro_rules! write {
    () => {
        # [cfg (any (feature = "std" , test))] pub mod write ;
    };
}

write!()