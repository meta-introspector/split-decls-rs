macro_rules! deps {
    () => {
        SampleUniform!();
        WeightedIndex!();
    };
}

macro_rules! WeightedIndexIter {
    () => {
        deps!();
        # [doc = " A lazy-loading iterator over the weights of a `WeightedIndex` distribution."] # [doc = " This is returned by [`WeightedIndex::weights`]."] pub struct WeightedIndexIter < 'a , X : SampleUniform + PartialOrd > { weighted_index : & 'a WeightedIndex < X > , index : usize , }
    };
}

WeightedIndexIter!()