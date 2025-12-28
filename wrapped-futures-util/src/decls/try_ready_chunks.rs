macro_rules! try_ready_chunks {
    () => {
        # [cfg (feature = "alloc")] mod try_ready_chunks ;
    };
}

try_ready_chunks!();