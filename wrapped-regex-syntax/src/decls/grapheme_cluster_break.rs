macro_rules! grapheme_cluster_break {
    () => {
        # [cfg (feature = "unicode-segment")] pub mod grapheme_cluster_break ;
    };
}

grapheme_cluster_break!();