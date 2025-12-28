macro_rules! lazy_buffer {
    () => {
        # [cfg (feature = "use_alloc")] mod lazy_buffer ;
    };
}

lazy_buffer!();