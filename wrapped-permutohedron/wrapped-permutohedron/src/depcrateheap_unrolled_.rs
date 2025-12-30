// Generated macro for heap_unrolled_ (function)
macro_rules! Depcrateheap_unrolled_ {
() => {
// Module: crate
// Provides: {"heap_unrolled_"}
// Dependencies: {}
# [doc = " Unrolled version of heap's algorithm due to Sedgewick"] fn heap_unrolled_ < T , F , C > (n : usize , xs : & mut [T] , f : & mut F) -> C where F : FnMut (& mut [T]) -> C , C : ControlFlow , { debug_assert ! (n >= 3) ; match n { 3 => { try_control ! (f (xs)) ; xs . swap (0 , 1) ; try_control ! (f (xs)) ; xs . swap (0 , 2) ; try_control ! (f (xs)) ; xs . swap (0 , 1) ; try_control ! (f (xs)) ; xs . swap (0 , 2) ; try_control ! (f (xs)) ; xs . swap (0 , 1) ; f (xs) } n => { for i in 0 .. n - 1 { try_control ! (heap_unrolled_ (n - 1 , xs , f)) ; let j = if n % 2 == 0 { i } else { 0 } ; xs . swap (j , n - 1) ; } heap_unrolled_ (n - 1 , xs , f) } } }
};
}
