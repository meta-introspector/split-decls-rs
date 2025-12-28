macro_rules! deps {
    () => {
        Token!();
    };
}

macro_rules! SliderHeuristic {
    () => {
        deps!();
        pub trait SliderHeuristic { fn best_slider_end (& mut self , tokens : & [Token] , hunk : Range < u32 > , earliest_end : u32) -> u32 ; }
    };
}

SliderHeuristic!();