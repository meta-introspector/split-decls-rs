// Generated macro for impl_378 (impl)
macro_rules! Depcrate_expand_typetreeimpl_378 {
() => {
// Module: crate::expand::typetree
// Provides: {"impl_378"}
// Dependencies: {}
impl TypeTree { pub fn new () -> Self { Self (Vec :: new ()) } pub fn all_ints () -> Self { Self (vec ! [Type { offset : - 1 , size : 1 , kind : Kind :: Integer , child : TypeTree :: new () }]) } pub fn int (size : usize) -> Self { let mut ints = Vec :: with_capacity (size) ; for i in 0 .. size { ints . push (Type { offset : i as isize , size : 1 , kind : Kind :: Integer , child : TypeTree :: new () , }) ; } Self (ints) } }
};
}
