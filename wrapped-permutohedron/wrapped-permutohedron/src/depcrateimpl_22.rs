// Generated macro for impl_22 (impl)
macro_rules! Depcrateimpl_22 {
() => {
// Module: crate
// Provides: {"impl_22"}
// Dependencies: {}
# [cfg (feature = "std")] # [doc = " Iterate the permutations"] # [doc = ""] # [doc = " **Note:** You can also generate the permutations lazily by using"] # [doc = " `.next_permutation()`."] impl < 'a , Data , T > Iterator for Heap < 'a , Data , T > where Data : ? Sized + AsMut < [T] > + ToOwned , { type Item = Data :: Owned ; fn next (& mut self) -> Option < Self :: Item > { self . next_permutation () . map (| perm | perm . to_owned ()) } }
};
}
