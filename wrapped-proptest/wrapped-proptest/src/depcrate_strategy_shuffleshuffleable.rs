// Generated macro for Shuffleable (trait)
macro_rules! Depcrate_strategy_shuffleShuffleable {
() => {
// Module: crate::strategy::shuffle
// Provides: {"Shuffleable"}
// Dependencies: {}
# [doc = " A value which can be used with the `prop_shuffle` combinator."] # [doc = ""] # [doc = " This is not a general-purpose trait. Its methods are prefixed with"] # [doc = " `shuffle_` to avoid the compiler suggesting them or this trait as"] # [doc = " corrections in errors."] pub trait Shuffleable { # [doc = " Return the length of this collection."] fn shuffle_len (& self) -> usize ; # [doc = " Swap the elements at the given indices."] fn shuffle_swap (& mut self , a : usize , b : usize) ; }
};
}
