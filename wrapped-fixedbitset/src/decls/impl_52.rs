macro_rules! deps {
    () => {
        FixedBitSet!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        # [doc = " Return **true** if the bit is enabled in the bitset,"] # [doc = " or **false** otherwise."] # [doc = ""] # [doc = " Note: bits outside the capacity are always disabled, and thus"] # [doc = " indexing a FixedBitSet will not panic."] impl Index < usize > for FixedBitSet { type Output = bool ; # [inline] fn index (& self , bit : usize) -> & bool { if self . contains (bit) { & true } else { & false } } }
    };
}

impl_52!()