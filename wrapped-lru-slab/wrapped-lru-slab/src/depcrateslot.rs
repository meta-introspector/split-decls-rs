// Generated macro for Slot (struct)
macro_rules! DepcrateSlot {
() => {
// Module: crate
// Provides: {"Slot"}
// Dependencies: {}
# [derive (Clone)] struct Slot < T > { value : Option < T > , # [doc = " Next slot in the LRU or free list"] next : u32 , # [doc = " Previous slot in the LRU list; NONE when free"] prev : u32 , }
};
}
