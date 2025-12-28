macro_rules! winapi {
    () => {
        # [cfg (windows)] # [macro_use] mod winapi ;
    };
}

winapi!();