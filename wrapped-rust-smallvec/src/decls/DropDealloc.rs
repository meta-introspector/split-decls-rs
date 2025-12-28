macro_rules! DropDealloc {
    () => {
        struct DropDealloc { ptr : NonNull < u8 > , size_bytes : usize , align : usize , }
    };
}

DropDealloc!()