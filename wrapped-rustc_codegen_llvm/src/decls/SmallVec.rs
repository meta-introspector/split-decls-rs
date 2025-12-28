macro_rules! SmallVec {
    () => {
        type SmallVec < T > = smallvec :: SmallVec < [T ; 16] > ;
    };
}

SmallVec!()