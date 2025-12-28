macro_rules! IndexAllocator {
    () => {
        pub (crate) struct IndexAllocator { next_const_index : isize , explicit : bool , }
    };
}

IndexAllocator!();