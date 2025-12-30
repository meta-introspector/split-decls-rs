// Generated macro for VecStrategy (struct)
macro_rules! Depcrate_collectionVecStrategy {
() => {
// Module: crate::collection
// Provides: {"VecStrategy"}
// Dependencies: {}
# [doc = " Strategy to create `Vec`s with a length in a certain range."] # [doc = ""] # [doc = " Created by the `vec()` function in the same module."] # [must_use = "strategies do nothing unless used"] # [derive (Clone , Debug)] pub struct VecStrategy < T : Strategy > { element : T , size : SizeRange , }
};
}
