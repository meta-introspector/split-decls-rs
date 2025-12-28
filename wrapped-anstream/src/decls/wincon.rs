macro_rules! wincon {
    () => {
        # [cfg (all (windows , feature = "wincon"))] mod wincon ;
    };
}

wincon!()