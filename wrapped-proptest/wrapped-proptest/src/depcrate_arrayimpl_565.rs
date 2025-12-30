// Generated macro for impl_565 (impl)
macro_rules! Depcrate_arrayimpl_565 {
() => {
// Module: crate::array
// Provides: {"impl_565"}
// Dependencies: {}
impl < S : Strategy , const N : usize > Strategy for [S ; N] { type Tree = ArrayValueTree < [S :: Tree ; N] > ; type Value = [S :: Value ; N] ; fn new_tree (& self , runner : & mut TestRunner) -> NewTree < Self > { Ok (ArrayValueTree { tree : unarray :: build_array_result (| i | self [i] . new_tree (runner)) ? , shrinker : 0 , last_shrinker : None , }) } }
};
}
