macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! zero_size_items {
    () => {
        deps!();
        # [test] fn zero_size_items () { SmallVec :: < () , 0 > :: new () . push (()) ; }
    };
}

zero_size_items!()