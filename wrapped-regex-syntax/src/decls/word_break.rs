macro_rules! word_break {
    () => {
        # [cfg (feature = "unicode-segment")] pub mod word_break ;
    };
}

word_break!()