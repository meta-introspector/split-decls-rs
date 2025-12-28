macro_rules! buffer {
    () => {
        # [cfg (feature = "alloc")] mod buffer ;
    };
}

buffer!();