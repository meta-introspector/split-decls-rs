macro_rules! writeable {
    () => {
        # [cfg (feature = "writeable")] mod writeable ;
    };
}

writeable!();