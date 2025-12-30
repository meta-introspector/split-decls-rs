// Generated macro for collect_from_iter (function)
macro_rules! Depcrate_testscollect_from_iter {
() => {
// Module: crate::tests
// Provides: {"collect_from_iter"}
// Dependencies: {}
# [test] fn collect_from_iter () { struct IterNoHint < I : Iterator > (I) ; impl < I : Iterator > Iterator for IterNoHint < I > { type Item = I :: Item ; fn next (& mut self) -> Option < Self :: Item > { self . 0 . next () } } let iter = IterNoHint (std :: iter :: repeat (1u8) . take (1_000_000)) ; let _y : SmallVec < u8 , 1 > = SmallVec :: from_iter (iter) ; }
};
}
