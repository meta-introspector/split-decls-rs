macro_rules! try_chunks {
    () => {
        # [cfg (feature = "alloc")] mod try_chunks ;
    };
}

try_chunks!();