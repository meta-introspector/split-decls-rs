macro_rules! DebugInfoOffset {
    () => {
        struct DebugInfoOffset < T > { # [doc = " Offset from the `base` used to calculate the debuginfo offset."] direct_offset : Size , # [doc = " Each offset in this vector indicates one level of indirection from the base or previous"] # [doc = " indirect offset plus a dereference."] indirect_offsets : Vec < Size > , # [doc = " The final location debuginfo should point to."] result : T , }
    };
}

DebugInfoOffset!();