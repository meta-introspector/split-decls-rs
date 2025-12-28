macro_rules! deps {
    () => {
        PropertyCodePointSet!();
    };
}

macro_rules! macro_304 {
    () => {
        deps!();
        icu_provider :: data_marker ! (# [doc = " `PropertyBinaryFullCompositionExclusionV1`"] PropertyBinaryFullCompositionExclusionV1 , PropertyCodePointSet <'static >, is_singleton = true) ;
    };
}

macro_304!()