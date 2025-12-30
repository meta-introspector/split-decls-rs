// Generated macro for tests (module)
macro_rules! Depcrate_arch_all_packedpairtests {
() => {
// Module: crate::arch::all::packedpair
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn forward_packedpair () { fn find (haystack : & [u8] , needle : & [u8] , _index1 : u8 , _index2 : u8 ,) -> Option < Option < usize > > { let f = Finder :: new (needle) ? ; Some (f . find_prefilter (haystack)) } crate :: tests :: packedpair :: Runner :: new () . fwd (find) . run () } }
};
}
