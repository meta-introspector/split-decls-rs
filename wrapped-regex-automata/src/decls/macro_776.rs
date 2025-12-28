macro_rules! deps {
    () => {
        PatternID!();
    };
}

macro_rules! macro_776 {
    () => {
        deps!();
        index_type_impls ! (PatternID , PatternIDError , PatternIDIter , WithPatternIDIter) ;
    };
}

macro_776!()