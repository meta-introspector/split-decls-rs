// Generated macro for heap_recursive (function)
macro_rules! Depcrateheap_recursive {
() => {
// Module: crate
// Provides: {"heap_recursive"}
// Dependencies: {}
# [doc = " Heap's algorithm for generating permutations, recursive version."] # [doc = ""] # [doc = " The recursive algorithm supports slices of any size (even though"] # [doc = " only a small number of elements is practical), and is generally"] # [doc = " a bit faster than the iterative version."] # [doc = ""] # [doc = " The closure `f` may return either `()` to simply run through all"] # [doc = " permutations, or a `Control` value that permits breaking the"] # [doc = " iteration early."] # [doc = ""] # [doc = " ```"] # [doc = " use permutohedron::heap_recursive;"] # [doc = ""] # [doc = " let mut data = [1, 2, 3, 4, 5, 6];"] # [doc = " let mut permutations = Vec::new();"] # [doc = " heap_recursive(&mut data, |permutation| {"] # [doc = "     permutations.push(permutation.to_vec())"] # [doc = " });"] # [doc = ""] # [doc = " assert_eq!(permutations.len(), 720);"] # [doc = " ```"] pub fn heap_recursive < T , F , C > (xs : & mut [T] , mut f : F) -> C where F : FnMut (& mut [T]) -> C , C : ControlFlow , { match xs . len () { 0 | 1 => f (xs) , 2 => { try_control ! (f (xs)) ; xs . swap (0 , 1) ; f (xs) } n => heap_unrolled_ (n , xs , & mut f) , } }
};
}
