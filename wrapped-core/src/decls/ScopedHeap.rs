macro_rules! ScopedHeap {
    () => {
        # [doc (hidden)] # [repr (C)] pub struct ScopedHeap { pub vtable : * const c_void , pub this : * const c_void , }
    };
}

ScopedHeap!()