macro_rules! macro_480 {
    () => {
        bitflags ! { # [repr (transparent)] # [derive (Default)] pub (crate) struct AllocKindFlags : u64 { const Unknown = 0 ; const Alloc = 1 ; const Realloc = 1 << 1 ; const Free = 1 << 2 ; const Uninitialized = 1 << 3 ; const Zeroed = 1 << 4 ; const Aligned = 1 << 5 ; } }
    };
}

macro_480!();