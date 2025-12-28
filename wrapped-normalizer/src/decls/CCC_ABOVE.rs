macro_rules! deps {
    () => {
        CanonicalCombiningClass!();
    };
}

macro_rules! CCC_ABOVE {
    () => {
        deps!();
        const CCC_ABOVE : CanonicalCombiningClass = ccc ! (Above , 230) ;
    };
}

CCC_ABOVE!()