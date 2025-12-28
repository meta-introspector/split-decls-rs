macro_rules! maybe_done {
    () => {
        # [cfg (feature = "alloc")] mod maybe_done ;
    };
}

maybe_done!();