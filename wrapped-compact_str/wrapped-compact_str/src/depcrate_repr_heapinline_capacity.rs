// Generated macro for inline_capacity (module)
macro_rules! Depcrate_repr_heapinline_capacity {
() => {
// Module: crate::repr::heap
// Provides: {"inline_capacity"}
// Dependencies: {}
mod inline_capacity { use core :: { alloc , ptr } ; use super :: { do_alloc , StrBuffer } ; use crate :: ReserveError ; # [doc = " # SAFETY:"] # [doc = " * `capacity` must be > 0"] pub (crate) unsafe fn alloc (capacity : usize) -> Result < ptr :: NonNull < u8 > , ReserveError > { do_alloc (layout (capacity)) } # [doc = " Deallocates a pointer which references a `HeapBuffer` whose capacity is stored inline"] # [doc = ""] # [doc = " # Safety"] # [doc = " * `ptr` must point to the start of a `HeapBuffer` whose capacity is on the inline"] pub (crate) unsafe fn dealloc (ptr : ptr :: NonNull < u8 > , capacity : usize) { let layout = layout (capacity) ; :: alloc :: alloc :: dealloc (ptr . as_ptr () , layout) ; } # [repr (C)] struct HeapBufferInnerInlineCapacity { buffer : StrBuffer , } # [inline (always)] pub (crate) fn layout (capacity : usize) -> alloc :: Layout { let buffer_layout = alloc :: Layout :: array :: < u8 > (capacity) . expect ("valid capacity") ; alloc :: Layout :: new :: < HeapBufferInnerInlineCapacity > () . extend (buffer_layout) . expect ("valid layout") . 0 . pad_to_align () } }
};
}
