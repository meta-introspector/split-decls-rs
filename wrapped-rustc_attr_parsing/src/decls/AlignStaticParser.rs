macro_rules! deps {
    () => {
        AlignParser!();
    };
}

macro_rules! AlignStaticParser {
    () => {
        deps!();
        # [derive (Default)] pub (crate) struct AlignStaticParser (AlignParser) ;
    };
}

AlignStaticParser!();