// Generated macro for heap_capacity (module)
macro_rules! Depcrate_repr_heapheap_capacity {
() => {
// Module: crate::repr::heap
// Provides: {"heap_capacity"}
// Dependencies: {}
mod heap_capacity { use core :: { alloc , ptr } ; use super :: { do_alloc , StrBuffer } ; use crate :: ReserveError ; # [doc = " SAFETY: `capacity` must not be zero"] pub (crate) unsafe fn alloc (capacity : usize) -> Result < ptr :: NonNull < u8 > , ReserveError > { do_alloc (layout (capacity)) } # [doc = " Deallocates a pointer which references a `HeapBuffer` whose capacity is on the heap"] # [doc = ""] # [doc = " # Safety"] # [doc = " * `ptr` must point to the start of a `HeapBuffer` whose capacity is on the heap. i.e. we"] # [doc = "   must have `ptr -> [cap<usize> ; string<bytes>]`"] pub (crate) unsafe fn dealloc (ptr : ptr :: NonNull < u8 > , capacity : usize) { let layout = layout (capacity) ; :: alloc :: alloc :: dealloc (ptr . as_ptr () , layout) ; } # [repr (C)] struct HeapBufferInnerHeapCapacity { capacity : usize , buffer : StrBuffer , } # [inline (always)] pub (crate) fn layout (capacity : usize) -> alloc :: Layout { let buffer_layout = alloc :: Layout :: array :: < u8 > (capacity) . expect ("valid capacity") ; alloc :: Layout :: new :: < HeapBufferInnerHeapCapacity > () . extend (buffer_layout) . expect ("valid layout") . 0 . pad_to_align () } }
};
}
