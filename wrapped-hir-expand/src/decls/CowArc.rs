macro_rules! CowArc {
    () => {
        enum CowArc < T > { Arc (Arc < T >) , Owned (T) , }
    };
}

CowArc!();