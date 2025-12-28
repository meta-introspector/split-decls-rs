macro_rules! deps {
    () => {
        GetDisjointMutError!();
    };
}

macro_rules! impl_198 {
    () => {
        deps!();
        impl core :: fmt :: Display for GetDisjointMutError { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { let msg = match self { GetDisjointMutError :: IndexOutOfBounds => "an index is out of bounds" , GetDisjointMutError :: OverlappingIndices => "there were overlapping indices" , } ; core :: fmt :: Display :: fmt (msg , f) } }
    };
}

impl_198!();