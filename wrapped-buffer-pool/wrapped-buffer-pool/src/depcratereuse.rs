// Generated macro for Reuse (trait)
macro_rules! DepcrateReuse {
() => {
// Module: crate
// Provides: {"Reuse"}
// Dependencies: {}
# [doc = " A trait that prepares an item to be returned to the pool. For example"] # [doc = " clearing it. `true` is returned if the item should be returned to the pool,"] # [doc = " `false` if it should be dropped."] pub trait Reuse { fn reuse (& mut self , trim : usize) -> bool ; }
};
}
