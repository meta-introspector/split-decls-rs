// Generated macro for DynPrecisionLimbVec (type)
macro_rules! Depcrate_ieeeDynPrecisionLimbVec {
() => {
// Module: crate::ieee
// Provides: {"DynPrecisionLimbVec"}
// Dependencies: {}
# [doc = " Growable `[Limb]` (i.e. heap-allocated and typically `Vec`/`SmallVec`/etc.),"] # [doc = " used only by algorithms that may require dynamically arbitrary precision,"] # [doc = " i.e. conversions from/to decimal strings."] # [doc = ""] # [doc = " Note: the specific type was chosen by starting with `SmallVec<[_; 1]>` and"] # [doc = " increasing the inline length as long as benchmarks were showing improvements,"] # [doc = " or at least the `Double::from_str` ones, which roughly had these behaviors:"] # [doc = " * `Vec<_>` -> `SmallVec<[_; 1]>`: ~15% speedup, but only for shorter inputs"] # [doc = " * `SmallVec<[_; 1]>` -> `SmallVec<[_; 2]>`: ~10% speedup for longer inputs"] # [doc = " * `SmallVec<[_; 2]>` -> `SmallVec<[_; 3]>`: noise and/or diminishing returns"] # [doc = ""] # [doc = " Note: the choice of type described above, and the factors in its decision,"] # [doc = " are specific to `Limb` being `u128`, so if `Limb` changes, this should too."] type DynPrecisionLimbVec = smallvec :: SmallVec < Limb , 2 > ;
};
}
