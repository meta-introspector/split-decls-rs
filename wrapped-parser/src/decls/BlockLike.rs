macro_rules! BlockLike {
    () => {
        # [derive (Clone , Copy , PartialEq , Eq)] enum BlockLike { Block , NotBlock , }
    };
}

BlockLike!()