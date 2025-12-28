macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! Context {
    () => {
        deps!();
        # [doc = " Additional context passed to the `inspect_object(…)` function of the [`Tree::traverse()`] method."] pub struct Context < 'a > { # [doc = " The pack entry describing the object"] pub entry : & 'a crate :: data :: Entry , # [doc = " The offset at which `entry` ends in the pack, useful to learn about the exact range of `entry` within the pack."] pub entry_end : u64 , # [doc = " The decompressed object itself, ready to be decoded."] pub decompressed : & 'a [u8] , # [doc = " The depth at which this object resides in the delta-tree. It represents the number of base objects, with 0 indicating"] # [doc = " an 'undeltified' object, and higher values indicating delta objects with the given number of bases."] pub level : u16 , }
    };
}

Context!();