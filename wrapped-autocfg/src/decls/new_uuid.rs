macro_rules! new_uuid {
    () => {
        # [doc = " Generates a numeric ID to use in probe crate names."] # [doc = ""] # [doc = " This attempts to be random, within the constraints of Rust 1.0 and no dependencies."] fn new_uuid () -> u64 { const FNV_OFFSET_BASIS : u64 = 0xcbf2_9ce4_8422_2325 ; const FNV_PRIME : u64 = 0x100_0000_01b3 ; let set : std :: collections :: HashSet < u64 > = (0 .. 256) . collect () ; let mut hash : u64 = FNV_OFFSET_BASIS ; for x in set { hash = (hash ^ x) . wrapping_mul (FNV_PRIME) ; } hash }
    };
}

new_uuid!()