macro_rules! byteset_create {
    () => {
        fn byteset_create (chars : & [char]) -> u64 { chars . iter () . fold (0 , | a , & ch | (1 << (ch as u8 & 0x3f)) | a) }
    };
}

byteset_create!();