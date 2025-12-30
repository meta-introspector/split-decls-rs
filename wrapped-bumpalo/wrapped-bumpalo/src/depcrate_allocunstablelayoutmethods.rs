// Generated macro for UnstableLayoutMethods (trait)
macro_rules! Depcrate_allocUnstableLayoutMethods {
() => {
// Module: crate::alloc
// Provides: {"UnstableLayoutMethods"}
// Dependencies: {}
pub trait UnstableLayoutMethods { fn padding_needed_for (& self , align : usize) -> usize ; fn repeat (& self , n : usize) -> Result < (Layout , usize) , LayoutErr > ; fn array < T > (n : usize) -> Result < Layout , LayoutErr > ; }
};
}
