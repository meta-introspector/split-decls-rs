macro_rules! write_tokens {
    () => {
        fn write_tokens (interner : & Interner < & [u8] > , tokens : & [Token] , out : & mut Vec < u8 > ,) { for token in tokens { out . extend_from_slice (interner [* token]) ; } }
    };
}

write_tokens!();