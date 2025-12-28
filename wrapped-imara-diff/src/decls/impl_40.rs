macro_rules! deps {
    () => {
        SliderHeuristic!();
        Score!();
        IndentLevel!();
        Token!();
        IndentHeuristic!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl < IndentOfToken : Fn (Token) -> IndentLevel > SliderHeuristic for IndentHeuristic < IndentOfToken > { fn best_slider_end (& mut self , tokens : & [Token] , hunk : Range < u32 > , earliest_end : u32) -> u32 { const MAX_SLIDING : u32 = 100 ; let mut top_slider_end = earliest_end ; if top_slider_end < hunk . start - 1 { top_slider_end = hunk . start - 1 ; } if hunk . end > top_slider_end + MAX_SLIDING { top_slider_end = hunk . end - MAX_SLIDING ; } let group_size = hunk . end - hunk . start ; let mut best_score = Score :: for_range (top_slider_end - group_size .. top_slider_end , tokens , & self . indent_of_token ,) ; let mut best_slider_end = top_slider_end ; for slider_end in (top_slider_end + 1) ..= hunk . end { let score = Score :: for_range (slider_end - group_size .. slider_end , tokens , & self . indent_of_token ,) ; if score . is_improvement_over (best_score) { best_score = score ; best_slider_end = slider_end ; } } best_slider_end } }
    };
}

impl_40!();