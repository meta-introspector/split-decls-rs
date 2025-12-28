macro_rules! ch_width {
    () => {
        # [cfg (not (feature = "unicode"))] fn ch_width (_ : char) -> usize { 1 }
    };
}

ch_width!();