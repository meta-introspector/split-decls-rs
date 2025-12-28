macro_rules! BlockSizes {
    () => {
        # [doc = " Trait implemented for supported block sizes, i.e. for types from `U1` to `U255`."] pub trait BlockSizes : ArraySize + sealed :: BlockSizes { }
    };
}

BlockSizes!()