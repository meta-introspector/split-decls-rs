macro_rules! tui {
    () => {
        # [cfg (feature = "render-tui")] # [doc = ""] pub mod tui ;
    };
}

tui!()