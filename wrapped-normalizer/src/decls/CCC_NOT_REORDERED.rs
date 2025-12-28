macro_rules! deps {
    () => {
        CanonicalCombiningClass!();
    };
}

macro_rules! CCC_NOT_REORDERED {
    () => {
        deps!();
        const CCC_NOT_REORDERED : CanonicalCombiningClass = ccc ! (NotReordered , 0) ;
    };
}

CCC_NOT_REORDERED!();