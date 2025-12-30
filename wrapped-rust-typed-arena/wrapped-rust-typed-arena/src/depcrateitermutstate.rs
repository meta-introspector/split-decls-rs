// Generated macro for IterMutState (enum)
macro_rules! DepcrateIterMutState {
() => {
// Module: crate
// Provides: {"IterMutState"}
// Dependencies: {}
enum IterMutState < 'a , T > { ChunkListRest { index : usize , inner_iter : slice :: IterMut < 'a , T > , } , ChunkListCurrent { iter : slice :: IterMut < 'a , T > , } , }
};
}
