macro_rules! gen_per_hasher_seed {
    () => {
        pub (crate) fn gen_per_hasher_seed () -> u64 { let mut per_hasher_seed = 0 ; let stack_ptr = core :: ptr :: addr_of ! (per_hasher_seed) as u64 ; per_hasher_seed = stack_ptr ; # [cfg (feature = "std")] { use std :: cell :: Cell ; thread_local ! { static PER_HASHER_NONDETERMINISM : Cell < u64 > = const { Cell :: new (0) } ; } PER_HASHER_NONDETERMINISM . with (| cell | { let nondeterminism = cell . get () ; per_hasher_seed = folded_multiply (per_hasher_seed , ARBITRARY1 ^ nondeterminism) ; cell . set (per_hasher_seed) ; }) } ; # [cfg (not (feature = "std"))] { use core :: sync :: atomic :: { AtomicUsize , Ordering } ; static PER_HASHER_NONDETERMINISM : AtomicUsize = AtomicUsize :: new (0) ; let nondeterminism = PER_HASHER_NONDETERMINISM . load (Ordering :: Relaxed) as u64 ; per_hasher_seed = folded_multiply (per_hasher_seed , ARBITRARY1 ^ nondeterminism) ; PER_HASHER_NONDETERMINISM . store (per_hasher_seed as usize , Ordering :: Relaxed) ; } folded_multiply (per_hasher_seed , ARBITRARY2) }
    };
}

gen_per_hasher_seed!()