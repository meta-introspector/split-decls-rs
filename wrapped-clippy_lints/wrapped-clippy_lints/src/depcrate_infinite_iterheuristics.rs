// Generated macro for HEURISTICS (const)
macro_rules! Depcrate_infinite_iterHEURISTICS {
() => {
// Module: crate::infinite_iter
// Provides: {"HEURISTICS"}
// Dependencies: {}
# [doc = " a slice of (method name, number of args, heuristic, bounds) tuples"] # [doc = " that will be used to determine whether the method in question"] # [doc = " returns an infinite or possibly infinite iterator. The finiteness"] # [doc = " is an upper bound, e.g., some methods can return a possibly"] # [doc = " infinite iterator at worst, e.g., `take_while`."] const HEURISTICS : [(Symbol , usize , Heuristic , Finiteness) ; 19] = [(sym :: zip , 1 , All , Infinite) , (sym :: chain , 1 , Any , Infinite) , (sym :: cycle , 0 , Always , Infinite) , (sym :: map , 1 , First , Infinite) , (sym :: by_ref , 0 , First , Infinite) , (sym :: cloned , 0 , First , Infinite) , (sym :: rev , 0 , First , Infinite) , (sym :: inspect , 0 , First , Infinite) , (sym :: enumerate , 0 , First , Infinite) , (sym :: peekable , 1 , First , Infinite) , (sym :: fuse , 0 , First , Infinite) , (sym :: skip , 1 , First , Infinite) , (sym :: skip_while , 0 , First , Infinite) , (sym :: filter , 1 , First , Infinite) , (sym :: filter_map , 1 , First , Infinite) , (sym :: flat_map , 1 , First , Infinite) , (sym :: unzip , 0 , First , Infinite) , (sym :: take_while , 1 , First , MaybeInfinite) , (sym :: scan , 2 , First , MaybeInfinite) ,] ;
};
}
