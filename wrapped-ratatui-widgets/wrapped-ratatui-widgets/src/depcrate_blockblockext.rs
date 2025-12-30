// Generated macro for BlockExt (trait)
macro_rules! Depcrate_blockBlockExt {
() => {
// Module: crate::block
// Provides: {"BlockExt"}
// Dependencies: {}
# [doc = " An extension trait for [`Block`] that provides some convenience methods."] # [doc = ""] # [doc = " This is implemented for [`Option<Block>`](Option) to simplify the common case of having a"] # [doc = " widget with an optional block."] pub trait BlockExt { # [doc = " Return the inner area of the block if it is `Some`. Otherwise, returns `area`."] # [doc = ""] # [doc = " This is a useful convenience method for widgets that have an `Option<Block>` field"] fn inner_if_some (& self , area : Rect) -> Rect ; }
};
}
