macro_rules! deps {
    () => {
        Block!();
    };
}

macro_rules! IntoOnes {
    () => {
        deps!();
        pub struct IntoOnes { bitset_front : Block , bitset_back : Block , block_idx_front : usize , block_idx_back : usize , remaining_blocks : core :: iter :: Copied < core :: slice :: Iter < 'static , usize > > , _buf : Vec < SimdBlock > , }
    };
}

IntoOnes!();