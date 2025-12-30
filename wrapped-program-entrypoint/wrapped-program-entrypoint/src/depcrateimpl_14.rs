// Generated macro for impl_14 (impl)
macro_rules! Depcrateimpl_14 {
() => {
// Module: crate
// Provides: {"impl_14"}
// Dependencies: {}
impl BumpAllocator { # [doc = " Creates the allocator tied to a provided slice."] # [doc = " This will not initialize the provided memory, except for the first"] # [doc = " bytes where the pointer is stored."] # [doc = ""] # [doc = " # Safety"] # [doc = " As long as BumpAllocator or any of its allocations are alive,"] # [doc = " writing into or deallocating the arena will cause UB."] # [doc = ""] # [doc = " Integer arithmetic in this global allocator implementation is safe when"] # [doc = " operating on the prescribed `HEAP_START_ADDRESS` and `HEAP_LENGTH`. Any"] # [doc = " other use may overflow and is thus unsupported and at one's own risk."] # [inline] # [allow (clippy :: arithmetic_side_effects)] pub unsafe fn new (arena : & mut [u8]) -> Self { debug_assert ! (arena . len () > size_of ::< usize > () , "Arena should be larger than usize") ; let pos_ptr = arena . as_mut_ptr () as * mut usize ; * pos_ptr = pos_ptr as usize + arena . len () ; Self { start : pos_ptr as usize , len : arena . len () , } } # [doc = " Creates the allocator tied to specific range of addresses."] # [doc = ""] # [doc = " # Safety"] # [doc = " This is unsafe in most situations, unless you are totally sure that the"] # [doc = " provided start address and length can be written to by the allocator,"] # [doc = " and that the memory will be usable for the lifespan of the allocator."] # [doc = ""] # [doc = " For Solana on-chain programs, a certain address range is reserved, so"] # [doc = " the allocator can be given those addresses."] pub const unsafe fn with_fixed_address_range (start : usize , len : usize) -> Self { Self { start , len } } }
};
}
