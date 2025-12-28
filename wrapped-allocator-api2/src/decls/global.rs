macro_rules! global {
    () => {
        # [cfg (feature = "alloc")] mod global ;
    };
}

global!()