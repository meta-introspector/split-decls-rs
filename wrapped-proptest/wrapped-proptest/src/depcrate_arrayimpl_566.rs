// Generated macro for impl_566 (impl)
macro_rules! Depcrate_arrayimpl_566 {
() => {
// Module: crate::array
// Provides: {"impl_566"}
// Dependencies: {}
impl < S : Strategy , const N : usize > Strategy for UniformArrayStrategy < S , [S :: Value ; N] > { type Tree = ArrayValueTree < [S :: Tree ; N] > ; type Value = [S :: Value ; N] ; fn new_tree (& self , runner : & mut TestRunner) -> NewTree < Self > { Ok (ArrayValueTree { tree : unarray :: build_array_result (| _ | { self . strategy . new_tree (runner) }) ? , shrinker : 0 , last_shrinker : None , }) } }
};
}
