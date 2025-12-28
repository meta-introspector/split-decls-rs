macro_rules! sentence_break {
    () => {
        # [cfg (feature = "unicode-segment")] pub mod sentence_break ;
    };
}

sentence_break!()