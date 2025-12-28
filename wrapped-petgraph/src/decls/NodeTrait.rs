macro_rules! NodeTrait {
    () => {
        # [doc = " A trait group for `GraphMap`'s node identifier."] pub trait NodeTrait : Copy + Ord + Hash { }
    };
}

NodeTrait!()