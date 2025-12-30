// Generated macro for Heap (struct)
macro_rules! DepcrateHeap {
() => {
// Module: crate
// Provides: {"Heap"}
// Dependencies: {}
# [doc = " Heap's algorithm for generating permutations."] # [doc = ""] # [doc = " An iterative method of generating all permutations of a sequence."] # [doc = ""] # [doc = " Note that for *n* elements there are *n!* (*n* factorial) permutations."] # [doc = ""] # [doc = " ```"] # [doc = " use permutohedron::Heap;"] # [doc = ""] # [doc = " let mut data = vec![1, 2, 3];"] # [doc = " let heap = Heap::new(&mut data);"] # [doc = ""] # [doc = " let mut permutations = Vec::new();"] # [doc = " for data in heap {"] # [doc = "     permutations.push(data.clone());"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(permutations.len(), 6);"] # [doc = " ```"] # [repr (C)] pub struct Heap < 'a , Data : 'a + ? Sized , T : 'a > { data : & 'a mut Data , n : u32 , c : [u8 ; MAXHEAP - 1] , _element : PhantomData < & 'a mut T > , }
};
}
