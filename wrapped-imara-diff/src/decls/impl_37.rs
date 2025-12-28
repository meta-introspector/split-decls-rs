macro_rules! deps {
    () => {
        SliderHeuristic!();
        Token!();
        NoSliderHeuristic!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl SliderHeuristic for NoSliderHeuristic { fn best_slider_end (& mut self , _tokens : & [Token] , hunk : Range < u32 > , _earliest_end : u32) -> u32 { hunk . end } }
    };
}

impl_37!();