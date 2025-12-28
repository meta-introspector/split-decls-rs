macro_rules! Zeroes {
    () => {
        # [doc = " An  iterator producing the indices of the set bit in a set."] # [doc = ""] # [doc = " This struct is created by the [`FixedBitSet::ones`] method."] pub struct Zeroes < 'a > { bitset : usize , block_idx : usize , len : usize , remaining_blocks : core :: slice :: Iter < 'a , usize > , }
    };
}

Zeroes!()