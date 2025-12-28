macro_rules! concurrent_stream {
    () => {
        # [cfg (feature = "alloc")] pub mod concurrent_stream ;
    };
}

concurrent_stream!()