macro_rules! write {
    () => {
        # [cfg (feature = "write")] pub mod write ;
    };
}

write!();