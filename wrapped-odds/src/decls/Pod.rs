macro_rules! deps {
    () => {
        UnalignedIter!();
    };
}

macro_rules! Pod {
    () => {
        deps!();
        # [doc = " \"plain old data\": Types that we can stick arbitrary bit patterns into,"] # [doc = " and thus use them as blocks in `split_aligned_for` or in `UnalignedIter`."] pub unsafe trait Pod : Copy { }
    };
}

Pod!();