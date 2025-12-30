// Generated macro for MiriAllocBytes (struct)
macro_rules! Depcrate_alloc_alloc_bytesMiriAllocBytes {
() => {
// Module: crate::alloc::alloc_bytes
// Provides: {"MiriAllocBytes"}
// Dependencies: {}
# [doc = " Allocation bytes that explicitly handle the layout of the data they're storing."] # [doc = " This is necessary to interface with native code that accesses the program store in Miri."] # [derive (Debug)] pub struct MiriAllocBytes { # [doc = " Stored layout information about the allocation."] layout : alloc :: Layout , # [doc = " Pointer to the allocation contents."] # [doc = " Invariant:"] # [doc = " * If `self.layout.size() == 0`, then `self.ptr` was allocated with the equivalent layout with size 1."] # [doc = " * Otherwise, `self.ptr` points to memory allocated with `self.layout`."] ptr : * mut u8 , # [doc = " Whether this instance of `MiriAllocBytes` had its allocation created by calling `alloc::alloc()`"] # [doc = " (`Global`) or the discrete allocator (`Isolated`)"] params : MiriAllocParams , }
};
}
