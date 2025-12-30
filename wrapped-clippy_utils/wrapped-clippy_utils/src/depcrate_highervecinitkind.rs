// Generated macro for VecInitKind (enum)
macro_rules! Depcrate_higherVecInitKind {
() => {
// Module: crate::higher
// Provides: {"VecInitKind"}
// Dependencies: {}
# [doc = " A parsed `Vec` initialization expression"] # [derive (Clone , Copy)] pub enum VecInitKind { # [doc = " `Vec::new()`"] New , # [doc = " `Vec::default()` or `Default::default()`"] Default , # [doc = " `Vec::with_capacity(123)`"] WithConstCapacity (u128) , # [doc = " `Vec::with_capacity(slice.len())`"] WithExprCapacity (HirId) , }
};
}
