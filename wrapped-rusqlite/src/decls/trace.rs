macro_rules! trace {
    () => {
        # [cfg (feature = "trace")] pub mod trace ;
    };
}

trace!()