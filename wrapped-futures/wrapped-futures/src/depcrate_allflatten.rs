// Generated macro for flatten (function)
macro_rules! Depcrate_allflatten {
() => {
// Module: crate::all
// Provides: {"flatten"}
// Dependencies: {}
# [test] fn flatten () { fn finished < T : Send + 'static > (a : T) -> Finished < T , u32 > { futures :: finished (a) } fn failed < E : Send + 'static > (b : E) -> Failed < i32 , E > { futures :: failed (b) } assert_done (| | finished (finished (1)) . flatten () , ok (1)) ; assert_done (| | finished (failed (1)) . flatten () , err (1)) ; assert_done (| | failed (1u32) . map (finished) . flatten () , err (1)) ; assert_done (| | futures :: finished :: < _ , u8 > (futures :: finished :: < _ , u32 > (1)) . flatten () , ok (1)) ; assert_empty (| | finished (empty :: < i32 , u32 > ()) . flatten ()) ; assert_empty (| | empty :: < i32 , u32 > () . map (finished) . flatten ()) ; }
};
}
