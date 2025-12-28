macro_rules! OffsetIter {
    () => {
        struct OffsetIter < 'a , T > { inner : Chunks < 'a , T > , offset : usize , }
    };
}

OffsetIter!()