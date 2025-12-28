macro_rules! block_xor {
    () => {
        fn block_xor (dst : & mut [u32] , src : & [u32] , n : usize) { for (i , elem) in src [.. n] . iter () . enumerate () { dst [i] ^= elem ; } }
    };
}

block_xor!();