// Generated macro for impl_55 (impl)
macro_rules! Depcrate_postprocessimpl_55 {
() => {
// Module: crate::postprocess
// Provides: {"impl_55"}
// Dependencies: {}
impl Diff { pub fn postprocess_with (& mut self , before : & [Token] , after : & [Token] , mut heuristic : impl SliderHeuristic ,) { Postprocessor { added : & mut self . added , removed : & mut self . removed , tokens : after , hunk : Hunk { before : 0 .. 0 , after : 0 .. 0 , } , heuristic : & mut heuristic , } . run () ; Postprocessor { added : & mut self . removed , removed : & mut self . added , tokens : before , hunk : Hunk { before : 0 .. 0 , after : 0 .. 0 , } , heuristic : & mut heuristic , } . run () } pub fn postprocess_with_heuristic < T > (& mut self , input : & InternedInput < T > , heuristic : impl SliderHeuristic ,) { self . postprocess_with (& input . before , & input . after , heuristic) ; } }
};
}
