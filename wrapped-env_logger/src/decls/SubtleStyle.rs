macro_rules! SubtleStyle {
    () => {
        # [cfg (not (feature = "color"))] type SubtleStyle = & 'static str ;
    };
}

SubtleStyle!()