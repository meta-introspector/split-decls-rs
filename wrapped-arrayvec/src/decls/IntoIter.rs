macro_rules! deps {
    () => {
        ArrayVec!();
    };
}

macro_rules! IntoIter {
    () => {
        deps!();
        # [doc = " By-value iterator for `ArrayVec`."] pub struct IntoIter < T , const CAP : usize > { index : usize , v : ArrayVec < T , CAP > , }
    };
}

IntoIter!();