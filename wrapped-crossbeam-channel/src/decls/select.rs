macro_rules! select {
    () => {
        # [cfg (feature = "std")] mod select ;
    };
}

select!()