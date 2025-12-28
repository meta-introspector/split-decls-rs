macro_rules! IndexIter {
    () => {
        pub (crate) struct IndexIter { iter : ops :: Range < usize > , offset : usize , }
    };
}

IndexIter!();