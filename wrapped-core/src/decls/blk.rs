macro_rules! blk {
    () => {
        const fn blk (block : & [u32] , i : usize) -> u32 { let value = block [(i + 13) & 15] ^ block [(i + 8) & 15] ^ block [(i + 2) & 15] ^ block [i] ; rol (value , 1) }
    };
}

blk!();