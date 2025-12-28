macro_rules! deps {
    () => {
        RawVec!();
        Allocator!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl < T , A : Allocator > Drop for RawVec < T , A > { # [doc = " Frees the memory owned by the `RawVec` *without* trying to drop its contents."] # [inline (always)] fn drop (& mut self) { if let Some ((ptr , layout)) = self . current_memory () { unsafe { self . alloc . deallocate (ptr , layout) } } } }
    };
}

impl_96!()