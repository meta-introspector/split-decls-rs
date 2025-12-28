macro_rules! deps {
    () => {
        Flags!();
    };
}

macro_rules! MAX_CANDIDATES {
    () => {
        deps!();
        const MAX_CANDIDATES : usize = std :: mem :: size_of :: < Flags > () * 8 ;
    };
}

MAX_CANDIDATES!();