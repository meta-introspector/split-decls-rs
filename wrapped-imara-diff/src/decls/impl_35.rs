macro_rules! deps {
    () => {
        SliderHeuristic!();
        Token!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl < F > SliderHeuristic for F where F : FnMut (& [Token] , Range < u32 > , u32) -> u32 , { fn best_slider_end (& mut self , tokens : & [Token] , hunk : Range < u32 > , earliest_end : u32) -> u32 { self (tokens , hunk , earliest_end) } }
    };
}

impl_35!()