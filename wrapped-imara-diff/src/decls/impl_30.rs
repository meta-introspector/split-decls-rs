macro_rules! deps {
    () => {
        InternedInput!();
        Postprocessor!();
        Hunk!();
        Diff!();
        SliderHeuristic!();
        Token!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl Diff { pub fn postprocess_with (& mut self , before : & [Token] , after : & [Token] , mut heuristic : impl SliderHeuristic ,) { Postprocessor { added : & mut self . added , removed : & mut self . removed , tokens : after , hunk : Hunk { before : 0 .. 0 , after : 0 .. 0 , } , heuristic : & mut heuristic , } . run () ; Postprocessor { added : & mut self . removed , removed : & mut self . added , tokens : before , hunk : Hunk { before : 0 .. 0 , after : 0 .. 0 , } , heuristic : & mut heuristic , } . run () } pub fn postprocess_with_heuristic < T > (& mut self , input : & InternedInput < T > , heuristic : impl SliderHeuristic ,) { self . postprocess_with (& input . before , & input . after , heuristic) ; } }
    };
}

impl_30!()