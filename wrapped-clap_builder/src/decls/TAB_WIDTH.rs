macro_rules! TAB_WIDTH {
    () => {
        # [cfg (feature = "help")] pub (crate) const TAB_WIDTH : usize = TAB . len () ;
    };
}

TAB_WIDTH!()