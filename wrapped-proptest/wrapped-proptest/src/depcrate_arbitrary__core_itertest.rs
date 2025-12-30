// Generated macro for test (module)
macro_rules! Depcrate_arbitrary__core_itertest {
() => {
// Module: crate::arbitrary::_core::iter
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; use std :: ops :: Range ; const DUMMY : & 'static [u8] = & [0 , 1 , 2 , 3 , 4] ; # [derive (Debug)] struct Dummy (u8) ; arbitrary ! (Dummy , SFnPtrMap < Range < u8 >, Self >; static_map (0 .. 5 , Dummy)) ; impl Iterator for Dummy { type Item = & 'static u8 ; fn next (& mut self) -> Option < Self :: Item > { if self . 0 < 5 { let r = & DUMMY [self . 0 as usize] ; self . 0 += 1 ; Some (r) } else { None } } } no_panic_test ! (empty => Empty < u8 >, once => Once < u8 >, repeat => Repeat < u8 >, cloned => Cloned < super :: Dummy >, cycle => Cycle < Once < u8 >>, enumerate => Enumerate < Repeat < u8 >>, fuse => Fuse < Once < u8 >>, peekable => Peekable < Repeat < u8 >>, rev => Rev <:: std :: vec :: IntoIter < u8 >>, zip => Zip < Repeat < u8 >, Repeat < u16 >>, chain => Chain < Once < u8 >, Once < u8 >>, skip => Skip < Repeat < u8 >>, take => Take < Repeat < u8 >>) ; # [cfg (feature = "unstable")] no_panic_test ! (step_by => StepBy < Repeat < u8 >>) ; }
};
}
