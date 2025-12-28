macro_rules! bstring {
    () => {
        # [cfg (feature = "alloc")] mod bstring ;
    };
}

bstring!()