macro_rules! write {
    () => {
        # [cfg (feature = "write_core")] pub mod write ;
    };
}

write!()